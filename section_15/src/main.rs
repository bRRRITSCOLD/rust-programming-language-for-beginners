extern crate section_15;

use std::env;
use std::process;

use section_15::Args;

fn main() {
  let unparsed_args: Vec<String> = env::args().collect();
  let parsed_args = Args::new(&unparsed_args).unwrap_or_else(|err| {
    eprintln!("Problem in parsing {} ", err);
    process::exit(1);
  });

  println!("Searching for {}", parsed_args.query);
  println!("In file {}", parsed_args.filename);

  section_15::run(parsed_args).unwrap_or_else(|err| {
    eprintln!("Application err {} ", err);
    process::exit(1);
  });
}
