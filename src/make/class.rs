use std::fs::read_to_string;
use crate::{TextInfo,database};
pub fn mk_it()->String {
    let page_01 = read_to_string("make/class/class.01").unwrap();
    let page_02 = read_to_string("make/class/class.02").unwrap();
    let page_03 = read_to_string("make/class/class.03").unwrap();
    let page_04 = read_to_string("make/class/class.04").unwrap();
    let page_05 = read_to_string("make/class/class.05").unwrap();
    format!("{}{}{}{}{}{}{}{}{}",page_01,mk_card_tj(),page_02,mk_card_js(),page_03,mk_card_sf(),page_04,mk_card_yj_sb(),page_05)
}
fn mk_card_tj() ->String{
    let page_01 = "\"><div class=\"class_card\">".to_string();
    let page_02 = "</div></a>".to_string();
    
    let mut cards = String::new();
    let list =database::get_list("data/info", "info").unwrap();
    let mut info_list = vec![];
    for i in list {
        let info = TextInfo::get(i).unwrap();
        info_list.push(info)
    }
    info_list.sort_by(|a, b| b.time.cmp(&a.time));
    for info in info_list {
        if info.mainclass =="题解"{
            cards += format!("<a href=\"/blog/{}{}{}|{}{}",info.id,page_01,info.title,info.date,page_02).as_str();
        }
    };
    cards
}
fn mk_card_yj_sb() ->String{
    let page_01 = "\"><div class=\"class_card\">".to_string();
    let page_02 = "</div></a>".to_string();
    let mut cards = String::new();
    let list =database::get_list("data/info", "info").unwrap();
    let mut info_list = vec![];
    for i in list {
        let info = TextInfo::get(i).unwrap();
        info_list.push(info)
    }
    info_list.sort_by(|a, b| b.time.cmp(&a.time));
    for info in info_list {
        if info.mainclass =="游记"||info.mainclass =="随笔"{
            cards += format!("<a href=\"/blog/{}{}{}|{}{}",info.id,page_01,info.title,info.date,page_02).as_str();
        }
    };
    cards
}
fn mk_card_js() ->String{
    let page_01 = "\"><div class=\"class_card\">".to_string();
    let page_02 = "</div></a>".to_string();
    let mut cards = String::new();
    let list =database::get_list("data/info", "info").unwrap();
    let mut info_list = vec![];
    for i in list {
        let info = TextInfo::get(i).unwrap();
        info_list.push(info)
    }
    info_list.sort_by(|a, b| b.time.cmp(&a.time));
    for info in info_list {
        if info.mainclass =="技术"{
            cards += format!("<a href=\"/blog/{}{}{}|{}{}",info.id,page_01,info.title,info.date,page_02).as_str();
        }
    };
    cards
}
fn mk_card_sf() ->String{
    let page_01 = "\"><div class=\"class_card\">".to_string();
    let page_02 = "</div></a>".to_string();
    let mut cards = String::new();
    let list =database::get_list("data/info", "info").unwrap();
    let mut info_list = vec![];
    for i in list {
        let info = TextInfo::get(i).unwrap();
        info_list.push(info)
    }
    info_list.sort_by(|a, b| b.time.cmp(&a.time));
    for info in info_list {
        if info.mainclass =="算法"{
            cards += format!("<a href=\"/blog/{}{}{}|{}{}",info.id,page_01,info.title,info.date,page_02).as_str();
        }
    };
    cards
}