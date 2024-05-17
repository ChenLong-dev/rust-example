use std::env;
use std::path::PathBuf;
use regex::Regex;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn get_path(target: String) -> String {
    let base_path:String = env::var("MY_ENV_DIR_VAR").unwrap_or_else(|e| {
        eprintln!("unable to read environment variables: {}", e);
        let current_path = env::current_dir().expect("unable to obtain the current path");
        println!("current_path: {:?}", current_path);
        format!("{}", current_path.display())
    });

    println!("base_path: {}", base_path);

    let re = Regex::new(r"[\\/]").unwrap(); // 使用正则表达式匹配“/”或“\”
    let parts: Vec<&str> = re.split(&base_path).collect(); // 分割字符串并收集到向量中

    let mut path_buf = PathBuf::new();
    for part in parts {
        path_buf.push(part);
    }
    path_buf.push(target);
    format!("{}", path_buf.display())
}

#[cfg(test)]
mod tools_tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_get_path() {
        let path = get_path(String::from("hello.html"));
        println!("xxx [test_get_html_path] path:{path}");
        if cfg!(windows) {
            assert_ne!(path, "".to_string());
        } else if cfg!(unix) {
            assert_ne!(path, "".to_string());
        }
    }
}
