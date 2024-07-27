use rocket::{figment::providers::{Format, Toml}, form::Form, fs::NamedFile, http::ContentType};
use std::{
    marker,
    path::{Path, PathBuf},
};
use web_server::{app, database, maker};

#[macro_use]
extern crate rocket;

#[get("/blog/<blog_id>")]
async fn blog(blog_id: String) -> Option<(ContentType, String)> {
    let page = maker::pages::make_page(blog_id);
    if page.is_none() {
        return None;
    }
    Some((ContentType::HTML, page.unwrap()))
}
#[get("/list")]
async fn blog_list() -> Option<(ContentType, String)> {
    let page = maker::pages::make_list_page();
    if page.is_none() {
        return None;
    }
    Some((ContentType::HTML, page.unwrap()))
}

#[get("/<file..>")]
async fn html(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("html/").join(file)).await.ok()
}
#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("html/index.html").await.ok()
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![blog, html, blog_list, index,app::new_text::new_content,app::remove::remove],
    )
}
