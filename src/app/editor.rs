use std::{
    path,
    time::{self, SystemTime},
};

use rocket::form::Form;

use crate::database;

use super::MarkdownDoc;

pub fn new(doc: Form<MarkdownDoc>) -> String {
    let id = mk_id();
    let file_name = format!("{}.htmldata", id);
    let html_status = database::add("data/html", file_name, doc.htmlcode.clone());
    let file_name = format!("{}.md", id);
    let md_status = database::add("data/md", file_name, doc.markdowncode.clone());
    let file_name = format!("{}.info", id);
    let info_status = database::add(
        "data/info",
        file_name,
        mk_info(&id, &doc.title, &doc.mainclass,doc.group.clone()),
    );
    format!(
        "{}\nHtmlCode:{}\nMarkdownCode:{}\nInfo_status:{}",
        id, html_status, md_status, info_status
    )
}
pub fn remove(id:String)->String{
    let file_name = format!("{}.htmldata", id);
    let html_status = database::remove("data/html", file_name);
    let file_name = format!("{}.md", id);
    let md_status = database::remove("data/md", file_name);
    let file_name = format!("{}.info", id);
    let info_status = database::remove("data/info", file_name);
    format!(
        "{}\nHtmlCode:{}\nMarkdownCode:{}\nInfo_status:{}",
        id, html_status, md_status, info_status
    )
}
fn mk_id() -> String {
    let mut id = String::new();
    id.push('B');
    while id.len() < 7 {
        let c: char = rand::random();
        if c >= 'A' && c <= 'Z' {
            id.push(c);
        }
    }
    let path = format!("./data/info/{}", id);
    let path = path::Path::new(&path);
    if path.exists() {
        mk_id()
    } else {
        id
    }
}
fn mk_info(id: &String ,title: &String,main_class:&String ,group_list: String) -> String {
    let date = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
    let time = match time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => 0,
    };
    let mut info = format!("{}\n{}\n{}\n{}\n{}", title, id, date, time,main_class);
    for i in group_list.split(' ') {
        info.push('\n');
        info += i;
    }
    info
}
