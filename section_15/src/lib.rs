use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub struct Args {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool
}

impl Args {
  pub fn new(args: &[String]) -> Result<Args, &'static str> {
    if args.len() < 3 {
      return Err("Not enough arguments")
    }

    let query = args[1].clone();
    let filename = args[2].clone();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Args{query, filename, case_sensitive})
  }
}

pub fn run(args: Args) -> Result<(), Box<Error>> {
  let mut f = File::open(args.filename)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  let results = if args.case_sensitive {
    search(&args.query, &contents)
  } else {
    search_case_insensitive(&args.query, &contents)
  };

  for line in results {
    println!("Found: {}", line);
  }

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
    Rust\nsafe,fast,productive.\nPick Three\nDuct Tape";
    
    assert_eq!(vec!["safe,fast,productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUst";
    let contents = "\
    Rust\nsafe,fast,productive.\nPick Three\nTrust Me";
    
    assert_eq!(vec!["Rust", "Trust Me"], search_case_insensitive(query, contents));
  }
}

pub fn search <'a>(query:&str,contents:&'a str)->Vec<&'a str> {
	let mut results=Vec::new();
	for line in contents.lines() {
		if line.contains(query){
			results.push(line);
		}
	}

	results
}

pub fn search_case_insensitive <'a>(query:&str,contents:&'a str)->Vec<&'a str> {
  let q = query.to_lowercase();
	let mut results=Vec::new();
	for line in contents.lines() {
		if line.to_lowercase().contains(&q){
			results.push(line);
		}
	}

	results
}
