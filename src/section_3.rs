use std::io;

fn section_three_even() {
	for n in 1..100 {
    if n%2 == 0 {
      println!("Even {}", n)
    }
  }
}

fn section_three_quiz_app() {
  let mut ans = String::new();
	println!("Tallest mountain in the world?");
  for n in 1.. 4 {
    ans.clear();
    io::stdin().read_line(&mut ans).expect("Failed.");
    ans = ans.trim().to_lowercase().to_string();
    if ans == "everest" {
      println!("Correct.");
      break;
    } else if n != 3 {
      println!("Try again.");
    } else {
      println!("You have failed.");
    }
  }
}

fn section_three_number_of_digits() {
  println!("{}", 123456789.to_string().len())
}


// fn section_two_loops_syntax_in_rust() {
// }

pub fn run() {
  println!("section 3 starting");

  section_three_even();

  section_three_quiz_app();

  section_three_number_of_digits();

  println!("section 3 finishing");
}