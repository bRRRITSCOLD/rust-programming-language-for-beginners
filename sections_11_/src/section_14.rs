#[derive(Debug)]
struct User {
  email: String,
  age: i32,
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  // fn area(self) -> u32 { this doesnt work because its not a
  // pointer and the ownership rule of rust gets rid of self if
  // not prepended with & to declare its a reference
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  fn new(width: u32, height: u32) -> Rectangle {
    if width < 1 || width > 100 {
      panic!("Width must be between 1 and 100")
    }

    Rectangle{
      width,
      height,
    }
  } 
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle{width: 8, height: 8};
    let smaller = Rectangle{width: 1, height: 5};
    let can_hold = larger.can_hold(&smaller);
    assert!(can_hold);
    assert_eq!(true, can_hold);
    assert_ne!(false, can_hold);
  }

  #[test]
  fn smaller_cannot_hold_larger() {
    let larger = Rectangle{width: 3, height: 3};
    let smaller = Rectangle{width: 4, height: 5};
    let can_hold = larger.can_hold(&smaller);
    assert!(!can_hold);
    assert_eq!(false, can_hold);
    assert_ne!(true, can_hold);
  }

  #[test]
  fn smaller_cannot_hold_larger_custom_error() {
    let larger = Rectangle{width: 3, height: 3};
    let smaller = Rectangle{width: 4, height: 5};
    let can_hold = larger.can_hold(&smaller);
    assert!(can_hold, "Should be false");
    assert_eq!(false, can_hold, "Should be false");
    assert_ne!(false, can_hold, "Should be false");
  }

  #[test]
  #[should_panic]
  fn new_should_panic() {
    Rectangle::new(101, 3);
  }
}


pub fn run() {
  println!("section 14 starting");

  println!("section 14 finishing");
}