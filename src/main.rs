use rocket::fs::NamedFile;
use web_server::app;
use std::path::{Path, PathBuf};


#[macro_use]
extern crate rocket;


#[get("/<file..>")]
async fn html(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("html/").join(file)).await.ok()
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("html/index.html")).await.ok()
}
#[get("/friend_link")]
async fn friend_link() -> Option<NamedFile> {
    NamedFile::open(Path::new("html/friend_link.html")).await.ok()
}
#[get("/about_me")]
async fn about_me() -> Option<NamedFile> {
    NamedFile::open(Path::new("html/about_me.html")).await.ok()
}
#[launch]
fn rocket() -> _ {
    
    rocket::build().mount(
        "/",
        routes![index,html,friend_link,about_me,app::new,app::remove,app::latest,app::blog,app::class],
    )
}
