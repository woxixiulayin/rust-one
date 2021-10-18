use clap::{ AppSettings, Clap };

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "haoke")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
  #[clap(subcommand)]
  subcmd: SubCommand,
}

// 标签联合体
#[derive(Clap, debug)]
enum SubCommand {
  Get(Get),
  Post(Post)
}

// get 子命令

fn main() {
    println!("Hello, world!");
}
