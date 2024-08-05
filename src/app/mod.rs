mod editor;
use rocket::{form::Form, get, http::ContentType, post, FromForm};

use crate::make;
#[derive(FromForm, Debug)]
pub struct MarkdownDoc{
    r#htmlcode:String,
    r#markdowncode: String,
    r#token:String,
    r#title:String,
    r#group:String,
    r#mainclass:String,
}
#[post("/editor/new", data = "<doc>")]  
pub async fn new(doc: Form<MarkdownDoc>) -> String {
    if !check_token(&doc.token){
        "Token Wrong".to_string()
    } else {
        editor::new(doc)
    }
    
}
#[derive(FromForm, Debug)]
pub struct IdCheck{
    r#token:String,
    r#id:String,
}
#[post("/editor/remove", data = "<doc>")]
pub async fn remove(doc:Form<IdCheck>)->String{
    if !check_token(&doc.token){
        "Token Wrong".to_string()
    } else {
        editor::remove(doc.id.clone())
    }
}
#[get("/latest")]
pub async fn latest()->Option<(ContentType,String)>{
    if let Some(page)  = make::latest::mk_it(){
        return Some((ContentType::HTML,page))
    }
    None
}
#[get("/class")]
pub async fn class()->(ContentType,String){
    (ContentType::HTML,make::class::mk_it())
}
#[get("/blog/<id>")]
pub async fn blog(id:String)->Option<(ContentType,String)>{
    if let Some(page)  = make::blog::mk_it(id){
        return Some((ContentType::HTML,page))
    }
    None
}
fn check_token(token:&String)->bool{
    sha256::digest(token) == ""
}