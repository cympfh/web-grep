extern crate structopt;
use structopt::StructOpt;

extern crate easy_scraper;
use easy_scraper::Pattern;

#[derive(StructOpt)]
#[structopt(name = "web-grep")]
struct Opts {
    #[structopt(
        name = "QUERY",
        help = "HTML Pattern Query (e.g. `<p>{}</p>`)",
        required = true
    )]
    query: String,

    #[structopt(name = "INPUT", help = "reading file", default_value = "-")]
    file: String,
}

fn cat(file_name: &String) -> String {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::{self, Read};

    let mut content = String::new();
    if file_name == "-" {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut content).unwrap();
    } else {
        let file = File::open(&file_name).unwrap();
        let mut buf_reader = BufReader::new(file);
        buf_reader.read_to_string(&mut content).unwrap();
    }
    content
}

fn main() {
    let opt = Opts::from_args();
    let pattern = Pattern::new(opt.query.replace("{}", "{{}}").as_str()).unwrap();
    let content = cat(&opt.file);
    for m in pattern.matches(content.as_str()) {
        if let Some(text) = m.get("") {
            println!("{}", text);
        }
    }
}
