use rocket::{form::Form,post,FromForm};
use sha256::digest;
use crate::database;
#[derive(FromForm, Debug)]
pub struct Re{
    r#token:String,
    r#id:String,
}
#[post("/removetext", data = "<tasks>")]  
pub async fn remove(tasks: Form<Re>)->String{
    let token_sha = digest(&tasks.token);
    if token_sha != "".to_string(){
        return "Token Wrong".to_string();
    }
    let mut re = String::from("Md:");
    let file_name = format!("{}.md",tasks.id);
    re += database::remove("data/text", file_name);
    let file_name = format!("{}.info",tasks.id);
    re += "\nInfo:";
    re += database::remove("data/text", file_name);
    let file_name = format!("{}.prehtml",tasks.id);
    re += "\nPrehtml:";
    re += database::remove("data/text", file_name);
    re
}