use clap::Parser;
use anyhow::Result;
use reqwest::Url;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "haoke")]

struct Opts {
  #[clap(subcommand)]
  subcmd: SubCommand,
}

// 标签联合体
#[derive(Parser, Debug)]
enum SubCommand {
  Get(Get),
  Post(Post)
}

// get 子命
#[derive(Parser, Debug)]
struct Get {
  #[clap(parse(try_from_str = parse_url))]
  url: String,
}

// post 子命令。需要输入一个url和若干个可选的 key=value，提供json形式的body
#[derive(Parser, Debug)]
struct Post {
  #[clap(parse(try_from_str = parse_url))]
  url: String,
  body: Vec<String>,
}

fn parse_url(s: &str) -> Result<String> {
  let _url: Url = s.parse()?;
  Ok(s.into())
}

fn main() {
  let opts: Opts = Opts::parse();
  println!("{:?}", opts);
}
