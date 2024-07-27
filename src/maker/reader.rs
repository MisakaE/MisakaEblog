use std::{fmt::Display, path};

use crate::{database, TextInfo};

pub fn read_info<T:Display>(id:T)->Option<TextInfo>{
    let file_name = format!("{}.info",id);
    if let Some(data) = database::get("data/text", file_name){
        let mut p = vec![];
        for i in data.split(|c|{c=='\n'||c=='\r'}){
            if i!=""{
                p.push(i.to_string())
            }
        }
        if p.len() < 3{
            return None;
        }
        let mut group = vec![];
        let title = p[0].to_string();
        let date = p[1].to_string();
        if let Ok(time) = p[2].parse::<u64>(){
            
            let id = id.to_string();
            for i in 3..p.len(){
            group.push(p[i].to_string())
            }
            return Some(TextInfo { title, id, date, group, time});
        }  
    };
    None
}
pub fn read_text<T:Display>(id:T)->Option<String>{
    let file_name = format!("{}.prehtml",id);
    database::get("data/text", file_name)
}
pub fn read_md<T:Display>(id:T)->Option<String>{
    let file_name = format!("{}.md",id);
    database::get("data/text", file_name)
}
pub fn read_name_list()->Option<Vec<String>>{
    if let Some(list) = database::get_list("data/text","info"){
        let mut name_list = vec![];
        for file_name in list{
            let path = path::Path::new(&file_name);
            if let Some(name) = path.file_stem(){
                let name = name.to_str().unwrap().to_string();
                name_list.push(name);
            }
        }
        return Some(name_list)
    }
    None
}