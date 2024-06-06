use std::{env, process};

use minigrep::{run, Config};


fn main() {
    println!("IGNORE_CASE=1 cargo run -- to poem.txt");

    // let args: Vec<String> = env::args().collect();
    // // 打印整个结构体
    // dbg!(&args);
    // println!("=== {:#?}", &args);

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
