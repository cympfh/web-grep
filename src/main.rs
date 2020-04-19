extern crate structopt;
use structopt::StructOpt;

extern crate easy_scraper;
use easy_scraper::Pattern;

#[derive(StructOpt)]
#[structopt(name = "web-grep")]
struct Opts {
    #[structopt(name = "query", help = "HTML Pattern Query (e.g. `<p>{}</p>`)", required = true)]
    query: String,
}

fn get_cat() -> String {
    let mut content = String::new();
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    loop {
        match stdin.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                content.push_str(&buffer);
                buffer.clear();
            }
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }
    content
}

fn main() {
    let opt = Opts::from_args();
    let pattern = Pattern::new(opt.query.replace("{}", "{{}}").as_str()).unwrap();
    let content = get_cat();
    for m in pattern.matches(content.as_str()) {
        if let Some(text) = m.get("") {
            println!("{}", text);
        }
    }
}
