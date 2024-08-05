pub mod database;
pub mod app;
pub mod make;
#[derive(Debug)]
pub struct TextInfo{
    pub title:String,
    pub id:String,
    pub date:String,
    pub time:u64,
    pub group:Vec<String>,
    pub mainclass:String,
}
impl TextInfo {
    pub fn get(file_name:String)->Option<Self>{
        let info = match database::get("data/info", file_name){
            Some(t) => t,
            None => return None
        };
        let mut info_list = vec![];
        for i in info.split(|c|{c=='\r'||c=='\n'}){
            if i != ""{
                info_list.push(i.to_string());
            }
        }
        if info_list.len()<5{
            return None;
        }
        let title = (&info_list[0]).to_string();
        let id = (&info_list[1]).to_string();
        let date = (&info_list[2]).to_string();
        let time :u64= match (&info_list[3]).parse(){
            Ok(t) => t,
            Err(_) =>0
        };
        let mut group = vec![];
        let mainclass = (&info_list[4]).to_string();
        for i in 5..info_list.len(){
            group.push((&info_list[i]).to_string())
        }
        Some(TextInfo{title,id,date,time,group,mainclass})
    }
}