use std::{fmt::Display, fs::{self, File},io::Write, path::Path};

use super::DataBaseError;

pub fn write_in<T:AsRef<Path>,C:Display>(path:T,text:C)->Result<(),DataBaseError<'static>>{
    let mut file = match File::create_new(path){
        Ok(f) => f,
        Err(_) => return Err(DataBaseError::WriteError("文件已经存在"))
    };
    match write!(file,"{}",text){
        Ok(_) => Ok(()),
        Err(_) => Err(DataBaseError::WriteError("文件已经创建，但无法写入"))
    }
    
}

pub fn get_file_list<T:AsRef<Path>>(path:T)->Result<Vec<String>,DataBaseError<'static>>{
    let dir = match fs::read_dir(path){
        Ok(t) => t,
        Err(_) => return Err(DataBaseError::FindError("读取目录失败"))
    };
    let mut list = vec![];
    for file in dir{
        if let Ok(file) = file{
            if file.path().is_file(){
                let name = file.file_name().to_str().unwrap().to_string();
                list.push(name);
            }
        }
    };
    Ok(list)
}

pub fn get_file_list_with_extension<T:AsRef<Path>,C:Display>(path:T,pat:C)->Result<Vec<String>,DataBaseError<'static>>{
    let list = get_file_list(path)?;
    let mut list_2 = vec![];
    for file in list{
        let path = Path::new(&file);
        if let Some(p) = path.extension(){
            if let Some(ps) = p.to_str(){
                if ps.to_string() == pat.to_string(){
                    list_2.push(file)
                }
            }
        }
    }
    Ok(list_2)
}

pub fn write_in_uncheck<T:AsRef<Path>,C:Display>(path:T,text:C)->Result<(),DataBaseError<'static>>{
    let mut file = match File::create(path){
        Ok(f) => f,
        Err(_) => return Err(DataBaseError::WriteError("文件打开或创建失败"))
    };
    match write!(file,"{}",text){
        Ok(_) => Ok(()),
        Err(_) => Err(DataBaseError::WriteError("文件已经创建，但无法写入"))
    }
    
}

pub fn remove<T:AsRef<Path>>(path:T)->&'static str{
    if !path.as_ref().exists(){
        return "该文件不存在"
    }
    if path.as_ref().is_dir(){
        return "该目标不是文件"
    }
    if path.as_ref().is_file(){
        match fs::remove_file(path){
            Ok(_) => return "删除成功",
            Err(_)=> return "该文件无法删除"
        };
    }
    "未能成功操作"
}

pub fn remove_uncheck<T:AsRef<Path>>(path:T){
    if !path.as_ref().exists(){
        return;
    }
    if path.as_ref().is_dir(){
        return;
    }
    if path.as_ref().is_file(){
        match fs::remove_file(path){
            Ok(_) => return,
            Err(_)=> return
        };
    }
}