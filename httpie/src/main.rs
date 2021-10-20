use std::str::FromStr;

use clap::Parser;
use anyhow::{ anyhow, Result};
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
  #[clap(parse(try_from_str = parse_kv_pair))]
  body: Vec<KvPair>,
}

#[derive(Debug)]
struct KvPair {
  k: String,
  v: String,
}

impl FromStr for KvPair {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // 使用 = 进行split,这会得到一个迭代器
    let mut split = s.split("=");
    let err = || anyhow!(format!("Failed to parse {}", s));
    Ok(Self {
        // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
        // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
        k: (split.next().ok_or_else(err)?).to_string(),
        // 从迭代器中取第二个结果作为value
        v: (split.next().ok_or_else(err)?).to_string(),
      })
  }
}

fn parse_url(s: &str) -> Result<String> {
  let _url: Url = s.parse()?;
  Ok(s.into())
}

// 因为我们为 KvPair 实现了 FromStr，这里可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
  Ok(s.parse()?)
}

fn main() {
  let opts: Opts = Opts::parse();
  println!("{:?}", opts);
}
