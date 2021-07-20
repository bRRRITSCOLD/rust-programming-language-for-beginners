use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;
use std::io;

fn section_twelve_unrecoverable_error_one() {
	let choice = "buy";
  println!("Whats your choice?");
  if choice == "buy" {
    println!("Thank you");
  } else {
    panic!("Please buy it");
  }
}

fn section_twelve_recoverable_error_one() {
	let f = File::open("hello.txt");
  match f {
    Ok(file) => {
      println!("File {:?}", file);
    },
    Err(err) => {
      println!("Error {:?}", err);
    }
  };
}

fn section_twelve_recoverable_error_two() {
	let f = File::open("hello.txt");
  match f {
    Ok(file) => {
      println!("File {:?}", file);
    },
    Err(ref err) if err.kind() == ErrorKind::NotFound => {
      match File::create("hello.txt") {
        Ok(file) => {
          println!("File {:?}", file);
        },
        Err(ref err) => {
          println!("Error creating file {:?}", err);
        }
      };
    }, Err(err) => {
      println!("Error opening file {:?}", err);
    }
  };
}

fn section_twelve_recoverable_error_three() {
	// let _f = File::open("hello.txt").unwrap();
	// let _f = File::open("hello.txt").expect("Failed");
}

fn section_twelve_recoverable_error_four() {
  fn read() -> Result<String, io::Error> {
    let f = File::open("asdf.txt");
    let mut f = match f {
      Ok(file) => file,
      Err(err) => return Err(err)
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
      Ok(_)=>Ok(s),
      Err(e)=>Err(e),
    }
  }

  let output = read();
  match output {
    Ok(fi) => println!("File {:?}", fi),
    Err(err) => println!("Err {:?}", err),
  }
}

fn section_twelve_recoverable_error_five() {
  fn read() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
  }

  let output = read();
  match output {
    Ok(fi) => println!("File {:?}", fi),
    Err(err) => println!("Err {:?}", err),
  }
}

fn section_twelve_assignment_one() {
  fn document() -> String {
    let doc = "Yes";
    println!("Do you have all documents");
    if doc == "Yes" {
      println!("{}\n", doc);
      return String::from("Yes");
    } else {
      println!("{}\n", doc);
      return String::from("No");
    }
  }

  fn fees() -> String {
    let fee = "Ok";
    println!("Do submit your fees");
    if fee == "Ok" {
      println!("{}\n", fee);
      return String::from("Yes");
    } else {
      println!("{}\n", fee);
      return String::from("No");
    }
  }

  println!("Welcome");
  let doc = document();
  if doc == String::from("Yes") {
    println!("Thank you for submitting documents.\n");
    println!("Now submit fees.\n");
    let f = fees();
    if f == String::from("Yes") {
      println!("Thank you for submitting fees.\n");
      println!("Admission confirmed.\n");
    } else {
      println!("Sorry you have not submitted fees.\n");
      println!("Admission cancelled.\n");
      panic!("");
    }
  } else {
    println!("Sorry you have not submitted documents.\n");
    println!("Admission cancelled.\n");
    panic!("");
  }
}

fn section_twelve_assignment_two() {
  println!("Input number\n");
  let mut no = String::new();
  io::stdin().read_line(&mut no).expect("Fail");
  let no: u32 = match no.trim().parse() {
    Ok(num) => num,
    Err(_) => 0,
  };
  println!("{}", no);
}

pub fn run() {
  println!("section 12 starting");

  section_twelve_unrecoverable_error_one();

  section_twelve_recoverable_error_one();

  section_twelve_recoverable_error_two();

  section_twelve_recoverable_error_three();

  section_twelve_recoverable_error_four();

  section_twelve_recoverable_error_five();

  section_twelve_assignment_one();

  section_twelve_assignment_two();

  println!("section 12 finishing");
}