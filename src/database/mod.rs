use std::{fmt::Display, fs, path::Path};
mod file_manager;
#[derive(Debug)]
pub enum DataBaseError<'a> {
    WriteError(&'a str),
    FindError(&'a str),
    ReadError(&'a str)
}
pub fn add<T:AsRef<Path>,I:Display>(path:T,file_name:I,text:String)->&'static str{
    let path = format!("{}/{}",path.as_ref().display(),file_name);
    match file_manager::write_in(path, text){
        Ok(_) => "Success",
        Err(_) => "Failure"
    }
}

pub fn remove<T:AsRef<Path>,I:Display>(path:T,file_name:I)->&'static str{
    let path = format!("{}/{}",path.as_ref().display(),file_name);
    match file_manager::remove(path) {
        "删除成功" => "Success",
        _ => "Failure"
    }
}

pub fn get<T:AsRef<Path>,I:Display>(path:T,file_name:I)->Option<String>{
    let path = format!("{}/{}",path.as_ref().display(),file_name);
    if let Ok(text) = fs::read_to_string(path){
        return Some(text)
    }
    None
}
pub fn get_list<T:AsRef<Path>,I:Display>(path:T,pat:I)->Option<Vec<String>>{
    if pat.to_string() == "".to_string(){
        if let Ok(v) = file_manager::get_file_list(path) {
            return Some(v)
        }
    } else{
        if let Ok(v) = file_manager::get_file_list_with_extension(path, pat){
            return Some(v)
        }
    };
    None
}
pub fn change<T:AsRef<Path>,I:Display>(path:T,file_name:I,text:String)->&'static str{
    let path = format!("{}/{}",path.as_ref().display(),file_name);
    match file_manager::write_in_uncheck(path, text){
        Ok(_) => "Success",
        Err(_) => "Failure"
    }
}
pub fn remove_uncheck<T:AsRef<Path>,I:Display>(path:T,file_name:I){
    let path = format!("{}/{}",path.as_ref().display(),file_name);
    file_manager::remove_uncheck(path)
}