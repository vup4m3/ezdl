use crate::aria2_shell::Aria2Shell;
use crate::aria2_shell::Entry;
use std::fs;

mod aria2_shell;

fn main() {
    let mut list = Vec::<Entry>::new();
    let mut dir = String::from("./");
    let mut i: u32 = 1;

    let data = fs::read_to_string("input.txt").expect("Can not read input.txt");
    let mut lines = data.lines();
    let str = lines.next();

    let path_str = str.unwrap().to_string();
    dir += &path_str;
    dir += "/";

    let str = lines.next();
    let file_name = str.unwrap().to_string();
    dir += &file_name;

    let str = lines.next();
    let ext = str.unwrap().to_string();

    loop {
        let str = lines.next();
        if str == None {
            break;
        }

        let url = str.unwrap().to_string();

        let mut out: String;
        out = file_name.to_string() + " - ";
        out += &i.to_string();
        out += ".";
        out += &ext.to_string();

        i += 1;

        list.push(Entry { url, out });
    }

    Aria2Shell::download(dir, list);
}
