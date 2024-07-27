use std::fs::read_to_string;
use super::reader::{self, read_text};
pub fn make_page(id: String) -> Option<String> {
    let text_info = match reader::read_info(&id){
        Some(t) => t,
        None =>return None
    };
    let text = match read_text(&id){
        Some(t) =>t ,
        None => return None
    };
    let page_01 = read_to_string("make/pages/page_01.half").unwrap();
    let page_02 = read_to_string("make/pages/page_02.half").unwrap();
    let page_03 = read_to_string("make/pages/page_03.half").unwrap();
    let page_04 = read_to_string("make/pages/page_04.half").unwrap();
    
    Some(format!(
        "{}{}{}{}{}{}{}",
        page_01, text_info.title, page_02, text_info.date, page_03, text, page_04
    ))
}
pub fn make_card(id: String, float: bool) -> Option<String> {
    let page_01 = read_to_string("make/list_card/list_card_01.half").unwrap();
    let page_02 = if float {
        read_to_string("make/list_card/list_card_02.half").unwrap()
    } else {
        read_to_string("make/list_card/list_card_02_1.half").unwrap()
    };
    let page_03 = read_to_string("make/list_card/list_card_03.half").unwrap();
    let page_04 = read_to_string("make/list_card/list_card_04.half").unwrap();
    let page_05 = read_to_string("make/list_card/list_card_05.half").unwrap();

    let text_info = match reader::read_info(&id){
        Some(t) => t,
        None =>return None
    };
    let mut group = String::new();
    for son in text_info.group {
        group += &son;
        group += " ";
    }
    Some(format!(
        "{}/blog/{}{}{}{}{}{}{}{}",
        page_01, text_info.id, page_02, text_info.title, page_03, group, page_04, text_info.date, page_05
    ))
}
pub fn make_list_page() -> Option<String> {
    let page_01 = read_to_string("make/list_page/list_page_01.half").unwrap();
    let page_02 = read_to_string("make/list_page/list_page_02.half").unwrap();
    let mut cards  = String::new();
    let mut f = true; 
    let mut file_list = match reader::read_name_list(){
        Some(t) => t,
        None => return None
    };
    list_time(&mut file_list);
    for id in file_list{
        
        if let Some(card) = make_card(id, f){
            cards += &card
        }
        if f {f = false}else{   
            f=true;
        };
    }
    Some(format!("{}{}{}",page_01,cards,page_02))
}
pub fn list_time(list:&mut Vec<String>){
    list.sort_by(|a,b|{
        let ai = reader::read_info(a).unwrap();
        let bi = reader::read_info(b).unwrap();
        ai.time.cmp(&bi.time)
    });
}