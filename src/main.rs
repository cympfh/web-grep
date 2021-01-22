extern crate structopt;
use structopt::StructOpt;

extern crate regex;
use regex::{Captures, Regex};

extern crate easy_scraper;
use easy_scraper::Pattern;

extern crate serde;
extern crate serde_json;

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

    #[structopt(
        long = "field-separator",
        short = "F",
        help = "output field separator",
        default_value = "\t"
    )]
    fs: String,

    #[structopt(long = "json", short = "j", help = "output as JSON")]
    json: bool,

    #[structopt(long = "debug", help = "NEVER need use this")]
    debug: bool,
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

fn replace_holders(query: &String) -> String {
    let pattern = Regex::new(r"\{([^\}]*)\}").unwrap();
    pattern
        .replace_all(query, |captures: &Captures| {
            format!("{{{{{}}}}}", &captures[1])
        })
        .to_string()
}

fn main() -> Result<(), String> {
    let opt = Opts::from_args();
    let query = replace_holders(&opt.query);
    if opt.debug {
        trace!(query);
    }
    let pattern = Pattern::new(&query)?;
    let content = cat(&opt.file);
    for m in pattern.matches(content.as_str()) {
        if opt.debug {
            trace!(m);
        }
        if opt.json {
            println!("{}", serde_json::to_string(&m).unwrap());
        } else if let Some(text) = m.get("") {
            println!("{}", text);
        } else if let Some(text) = m.get("1") {
            print!("{}", text);
            for i in 2..1024 {
                if let Some(text) = m.get(&i.to_string()) {
                    print!("{}{}", opt.fs, text);
                } else {
                    println!();
                    break;
                }
            }
        } else {
            let mut keys: Vec<_> = m.keys().collect();
            keys.sort();
            println!(
                "{}",
                keys.iter()
                    .map(|&key| m.get(key).unwrap())
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(&opt.fs)
            );
        }
    }
    Ok(())
}

#[macro_export]
macro_rules! trace {
    ($x:expr) => {
        eprintln!(">>> {} = {:?}", stringify!($x), $x)
    };
    ($($xs:expr),*) => { trace!(($($xs),*)) }
}
