pub mod database;
pub mod maker;
pub mod app;
#[derive(Debug)]
pub struct TextInfo{
    pub title:String,
    pub id:String,
    pub date:String,
    pub group:Vec<String>,
    pub time:u64,
}