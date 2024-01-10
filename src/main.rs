use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // 对 build 返回的 `Result` 进行处理
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // if let Err(e) = run(config) {
    //     println!("Application error: {e}");
    //     process::exit(1);
    // }
    match minigrep::run(config) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    }
}
