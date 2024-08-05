use crate::{database, TextInfo};
use std::{cmp::min, fs::read_to_string};
pub fn mk_it() -> Option<String> {
    let list = match database::get_list("data/info", "info") {
        Some(t) => t,
        None => return None,
    };
    let mut info_list = vec![];
    for i in list {
        let info = match TextInfo::get(i) {
            Some(t) => t,
            None => return None,
        };
        info_list.push(info)
    }
    info_list.sort_by(|a, b| b.time.cmp(&a.time));
    let page_01 = read_to_string("make/latest/latest.01").unwrap();
    let page_02 = read_to_string("make/latest/latest.02").unwrap();
    let mut cards = String::new();
    let mut f = true;
    for i in 0..min(info_list.len(),10){
        cards += mk_card(&info_list[i], f).as_str();
        if f {
            f=false;
        } else {
            f=true;
        }
    }
    Some(format!("{}{}{}",page_01,cards,page_02))
}

fn mk_card(info: &TextInfo, f: bool) ->String{
    let page_01 = read_to_string("make/card/card.01").unwrap();
    let page_02 = if f {
        read_to_string("make/card/card.02")
    } else {
        read_to_string("make/card/card_1.02")
    }.unwrap();
    let page_03 = read_to_string("make/card/card.03").unwrap();
    let page_04 = read_to_string("make/card/card.04").unwrap();
    let page_05 = read_to_string("make/card/card.05").unwrap();
    let mut text = String::new();
    for i in &info.group{
        text += i.as_str();
        text += " ";
    }
    format!(
        "{}{}{}{}{}{}{}{}{}",
        page_01,info.id ,page_02,info.title, page_03, text, page_04,info.date , page_05
    )
}
