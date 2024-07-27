use rocket::{form::Form,post,FromForm};
use sha256::digest;
use crate::maker;
#[derive(FromForm, Debug)]
pub struct Task{
    r#token:String,
    r#title: String,
    r#time: String,
    r#date: String,
    r#group: String,
    r#text: String,
}
#[post("/addtext", data = "<tasks>")]  
pub async fn new_content(tasks: Form<Task>) -> String {
    let token_sha = digest(&tasks.token);
    if token_sha != "".to_string(){
        return "Token Wrong".to_string();
    }
    let mut info = format!("{}\n{}\n{}", tasks.title, tasks.date, tasks.time);
    let groups: Vec<_> = tasks.group.split(' ').collect();
    for tag in groups {
        if tag == "" {
            continue;
        }
        info.push('\n');
        info += tag;
    }
    let id = maker::id_maker();
    format!(
        "Text:{}\nInfo:{}",
        maker::writer::write_in_text(&id, tasks.text.to_string()),
        maker::writer::write_in_info(&id,info)
    )
}