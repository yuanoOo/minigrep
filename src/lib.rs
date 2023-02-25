use std::{error::Error, fs, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read file
    let contents = fs::read_to_string(config.file_name)?;

    let result = match config.case_sensitive {
        true => search(&config.query, &contents),
        false => search_case_insentive(&config.query, &contents)    
    };

    for line in result{
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    query: String,
    file_name: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("Not Enough args!!!");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query: query,
            file_name: file_name,
            case_sensitive: case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = vec![];

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_case_insentive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = vec![];
    
    let query = query.to_lowercase();

    for line in contents.lines(){
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
        let contents = "\
Rust:
safe, fast, productive.
pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn one_result_case_insentive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.
Duct    ";

        assert_eq!(vec!["safe, fast, productive."], search_case_insentive(query, contents))
    }
}
