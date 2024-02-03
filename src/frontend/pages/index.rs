use askama::Template;


#[derive(Template)]
#[template(path="index.html")]
pub struct IndexTemplate {
    count: i32
}

static mut COUNT: i32 = 0;


pub async fn render_index() -> IndexTemplate {
    unsafe { COUNT += 1 };
    return IndexTemplate {
        count: unsafe { COUNT }
    }
}