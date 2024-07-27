use std::path::Path;

use rand::Rng;

pub mod reader;
pub mod pages;
pub mod writer;

pub fn id_maker()->String{
    let mut rng = rand::thread_rng();
    let mut id = String::from("B");
    while id.len() < 6{
        let c = rng.gen_range('A'..('Z' as u8 +1) as char);
        id.push(c)
    };
    let path = format!("data/text/{}.info",id);
    let path = Path::new(&path);
    if path.is_file() {
        id_maker()
    } else {
        id
    }
}