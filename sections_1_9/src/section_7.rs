use rand::Rng;
use std::io;

pub fn run() {
  println!("section 7 starting");

  section_seven_guessing_game();

  println!("section 7 finishing");
}

fn section_seven_guessing_game() {
  println!("Guess a number");

  let secret = rand::thread_rng().gen_range(0, 10);
  loop {
    println!("input your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("fail");
    let guess: i32 = guess.trim().parse().expect("fail");
    if guess == secret {
      println!("guessed correctly");
      break;
    } else {
      println!("try again");
      if guess > secret {
        println!("you have guessed a higher number");
      } else {
        println!("value is smalller");
      }
    }
  }
}