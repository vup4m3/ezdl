use std::io::Write;
use std::process::Command;

pub struct Entry {
    pub url: String,
    pub out: String,
}

pub struct Aria2Shell;

impl Aria2Shell {
    fn write_input_file(input_file:&str, list: Vec<Entry>) {
        let file = std::fs::File::create(input_file)
            .expect("Could not create file!");

        for e in &list {
            write!(&file, "{}\n", e.url)
                .expect("Could not write url!");
            write!(&file, "  out={}\n", e.out)
                .expect("could not write out!");
        }
    }
    pub fn download(dir: String, list: Vec<Entry>) {
        let input_file_name = String::from("tmp.txt");
        let aria2c = "aria2c";

        // write url/output list to tmp.txt
        Self::write_input_file(&input_file_name, list);

        // aria2c -i tmp.txt -d /dir
        println!(
            "{} -i {} -d {}",
            aria2c,
            &input_file_name,
            dir.to_string(),
        );

        Command::new(aria2c)
            .arg("-i")
            .arg(input_file_name)
            .arg("-d")
            .arg(dir.to_string())
            .output()
            .expect("aria2c calling error.");
    }
}