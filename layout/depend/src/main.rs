use std::env;
use utils;

fn main() {
    println!("=== Hello, depend! ===");
    
    env::set_var("MY_ENV_DIR_VAR", "x/y/z");
    
    let target = "xxx.txt".to_string();
    println!("utils::get_path: {}", utils::get_path(target));
}
