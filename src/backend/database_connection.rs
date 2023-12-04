use mongodb::{Client, options::{ClientOptions, ResolverConfig, Credential}};

pub async fn db_connection(client_uri: String)-> Result<Client,mongodb::error::Error>{
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await?;
    Client::with_options(options)
}