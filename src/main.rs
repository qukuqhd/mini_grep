use mini_grep::grep_lib::*;
use std::{env, error::Error, process::exit};
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("输入参数错误:{}", err);
        exit(1);
    });
    run(&config).unwrap_or_else(|err| {
        eprintln!("错误:{}", err.to_string());
        exit(1);
    });
    Ok(())
}
