use std::fmt::Display;

use crate::database;

pub fn write_in_text<T:Display>(id:T,text:String)->&'static str{
    let file_name= format!("{}.prehtml",id);
    database::add("data/text", file_name, text)
}
pub fn write_in_info<T:Display>(id:T,info:String)->&'static str{
    let file_name= format!("{}.info",id);
    database::add("data/text", file_name, info)
}