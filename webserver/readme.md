# 20240514【实例1：构建多线程 web server】
实例来源
```
https://kaisery.github.io/trpl-zh-cn/ch20-00-final-project-a-web-server.html
https://rust-lang.github.io/async-book/09_example/00_intro.html
```
运行进程命令
```
案例1：单线程webserver
cargo run --bin single_threaded_webserver

案例2：多线程webserver
cargo run --bin mult_threaded_webserver

案例3：优雅停机清理webserver
cargo run --bin grace_drop_webserver

案例4：异步webserver
cargo run --bin async_webserver
```
运行测试命令
```
测试1：tools_tests::test_get_html_path
cargo test tools_tests::test_get_html_path

测试2：async_tests::test_handle_connection
cargo test async_tests::test_handle_connection
```
编译
```
cargo build

export MY_ENV_HTML_DIR_VAR="abc/bcd"; ./async_webserver.exe
```