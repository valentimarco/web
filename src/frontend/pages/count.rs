use askama::Template;


#[derive(Template)]
#[template(path="count.html")]
pub struct CountTemplate {
    count: i32
}

static mut COUNT: i32 = 0;


pub async fn render_count() -> CountTemplate {
    unsafe { COUNT += 1 };
    return CountTemplate {
        count: unsafe { COUNT }
    }
}