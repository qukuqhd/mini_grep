pub mod grep_lib {
    use std::{env, error::Error, fs};

    pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
        match fs::read_to_string(config.file_path) {
            Ok(file_content) => {
                match config.ignore_case {
                    true => {
                        for line in search_case_insensitive(config.query_argument, &file_content) {
                            println!("{line}");
                        }
                    }
                    _ => {
                        for line in search(config.query_argument, &file_content) {
                            println!("{line}");
                        }
                    }
                }
                Ok(())
            }
            Err(err) => Err(Box::new(err)),
        }
    }
    pub struct Config<'a> {
        query_argument: &'a str,
        file_path: &'a str,
        ignore_case: bool,
    }
    impl<'a> Config<'a> {
        pub fn new(args: &'a Vec<String>) -> Result<Config, &'static str> {
            if args.len() < 3 {
                Err("输入的参数不够")
            } else {
                Ok(Config {
                    query_argument: &args[1],
                    file_path: &args[2],
                    ignore_case: env::var("IGNORE_CASE").is_ok(),
                })
            }
        }
    }
    pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
        let mut result = vec![];
        for line in content.lines() {
            if line.contains(query) {
                result.push(line);
            }
        }
        result
    }
    pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
        let query = query.to_lowercase();
        let mut result = vec![];
        for line in content.lines() {
            if line.to_lowercase().contains(&query) {
                result.push(line);
            }
        }
        result
    }
}
#[cfg(test)]
mod grep_lib_test {
    use crate::grep_lib::{search, search_case_insensitive};
    #[test]
    fn search_test() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn search_case_tets() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
