use std::{path::PathBuf, pin::Pin};

use axum::{Router,response::IntoResponse, extract::Request};
use hyper::body::Incoming;
use hyper_util::rt::{TokioIo, TokioExecutor};
use openssl::ssl::{SslAcceptor, SslMethod, SslFiletype, Ssl};
use tokio::net::TcpListener;
use tokio_openssl::SslStream;
use tower::Service;




mod frontend;
mod backend;


async fn fallback(uri: axum::http::Uri) -> impl IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri) )
}




//Semes like that merge fn don't accept a function that return the Router object, caused by askama_axum library

#[tokio::main]
async fn main(){
    env_logger::init();
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // let app = Router::new()
    //     .fallback(fallback)
    //     .nest("/api/v1",crate::backend::main_route::route_backend().await )
    //     .nest("/", frontend::route::route_frontend());
    // println!("Server open at: {}", "http://localhost:3000");
    // axum::serve(listener,app).await.unwrap()

    //https://certbot.eff.org/ to update the ssl certs
    let mut tls_builder = SslAcceptor::mozilla_modern_v5(SslMethod::tls()).unwrap();

    tls_builder
        .set_certificate_file(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("src/backend")
                .join("ssl_certs")
                .join("cert.pem"),
            SslFiletype::PEM,
        )
        .unwrap();

    tls_builder
        .set_private_key_file(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("src/backend")
                .join("ssl_certs")
                .join("key.pem"),
            SslFiletype::PEM,
        )
        .unwrap();

    tls_builder.check_private_key().unwrap();

    let tls_acceptor = tls_builder.build();

    let tcp_listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    log::info!("HTTPS server listening on localhost:3000. To contact curl -k https://localhost:3000");
    let app = Router::new()
        .fallback(fallback)
        .nest("/api/v1",crate::backend::main_route::route_backend().await )
        .nest("/", frontend::route::route_frontend());

    futures_util::pin_mut!(tcp_listener);

    loop {
        let tower_service = app.clone();
        let tls_acceptor = tls_acceptor.clone();

        // Wait for new tcp connection
        let (cnx, addr) = tcp_listener.accept().await.unwrap();

        tokio::spawn(async move {
            let ssl = Ssl::new(tls_acceptor.context()).unwrap();
            let mut tls_stream = SslStream::new(ssl, cnx).unwrap();
            if let Err(err) = SslStream::accept(Pin::new(&mut tls_stream)).await {
                log::error!(
                    "error during tls handshake connection from {}: {}",
                    addr, err
                );
                return;
            }

            // Hyper has its own `AsyncRead` and `AsyncWrite` traits and doesn't use tokio.
            // `TokioIo` converts between them.
            let stream = TokioIo::new(tls_stream);

            // Hyper has also its own `Service` trait and doesn't use tower. We can use
            // `hyper::service::service_fn` to create a hyper `Service` that calls our app through
            // `tower::Service::call`.
            let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
                // We have to clone `tower_service` because hyper's `Service` uses `&self` whereas
                // tower's `Service` requires `&mut self`.
                //
                // We don't need to call `poll_ready` since `Router` is always ready.
                tower_service.clone().call(request)
            });

            let ret = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
                .serve_connection_with_upgrades(stream, hyper_service)
                .await;

            if let Err(err) = ret {
                log::warn!("error serving connection from {}: {}", addr, err);
            }
        });
    }
    
}