use std::{fs, result};
use std::error::Error;
use std::env;

pub fn run(config : Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_name)?;

    let results = if config.case_sensitive{
        search(&config.query, &contents)
    } else{
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_name : String,
    pub case_sensitive : bool,
}
impl Config{
    pub fn new(args: &[String]) -> Result<Config, &str>{
        if (args.len()) < 3 {
            return Err(("Not enough parameter"));
        }
        let query = args[1].clone();
        let file_name = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query, file_name, case_sensitive})
}
}
pub fn search<'a>(query : &str, content : &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(
    query : &str,
    content : &'a str
    ) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results= Vec::new();
    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn one_result(){
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUst";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}