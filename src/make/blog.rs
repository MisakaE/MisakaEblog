use std::fs::read_to_string;

use crate::{database, TextInfo};
pub fn mk_it(id:String)->Option<String>{
    let page_01 = read_to_string("make/blog/blog.01").unwrap();
    let page_02 = read_to_string("make/blog/blog.02").unwrap();
    let page_03 = read_to_string("make/blog/blog.03").unwrap();
    let page_04 = read_to_string("make/blog/blog.04").unwrap();
    let file_name = format!("{}.htmldata",id);
    let text = match database::get("data/html", file_name){
        Some(t) => t,
        None => return None
    };
    let file_name = format!("{}.info",id);
    let info = match TextInfo::get(file_name){
        Some(t) => t,
        None => return None
    };
    Some(format!("{}{}{}{}{}{}{}",page_01,info.title,page_02,info.date,page_03,text,page_04))
}