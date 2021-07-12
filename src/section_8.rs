pub fn run() {
  println!("section 8 starting");

  section_eight_struct_one();

  section_eight_struct_two();

  section_eight_struct_three();

  section_eight_struct_four();

  section_eight_struct_five();

  section_eight_assignment();

  println!("section 8 finishing");
}

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
    Rectangle{
      width,
      height,
    }
  } 
}

fn section_eight_struct_one() {
  let user = User {
    email: String::from("user@email.com"),
    age: 21
  };
  println!("{:?}", user);
}

fn section_eight_struct_two() {
  let mut user = User {
    email: String::from("user@email.com"),
    age: 21
  };
  user.age = 22;
  println!("{:?}", user);
}

fn section_eight_struct_three() {
  let user = section_eight_struct_three_build(String::from("user@email.com"), 23);
  println!("{:?}", user);
}

fn section_eight_struct_three_build(email: String, age: i32) -> User {
  User {
    email,
    age,
  }
}

fn section_eight_struct_four() {
  let user1 = User {
    email: String::from("user1@email.com"),
    age: 21
  };
  let user2 = User {
    email: String::from("user2@email.com"),
    ..user1
  };
  println!("{:?} \n{:?}", user1, user2);
}

fn section_eight_struct_five() {
  let rectangle1 = Rectangle{
    width: 30,
    height: 50,
  };
  println!("{:?}", rectangle1);

  let rectangle1_area1 = rectangle1.area();
  println!("{}", rectangle1_area1);

  let rectangle1_area2 = rectangle1.area();
  println!("{}", rectangle1_area2);

  let rectangle2 = Rectangle{
    width: 20,
    height: 40,
  };
  println!("{:?}", rectangle2);

  let rectangle3 = Rectangle{
    width: 40,
    height: 60,
  };
  println!("{:?}", rectangle3);

  let rectangle1_can_hold_rectangle2 = rectangle1.can_hold(&rectangle2);
  println!("{}", rectangle1_can_hold_rectangle2);

  let rectangle1_can_hold_rectangle3 = rectangle1.can_hold(&rectangle3);
  println!("{}", rectangle1_can_hold_rectangle3);

  let rectangle4 = Rectangle::new(20, 30);
  println!("{:?}", rectangle4);
}

#[derive(Debug)]
struct Student {
  name: String,
  c: i32,
  java: i32,
  rust: i32,
}

impl Student {
  fn highest(&self) {
    if self.c > self.java && self.c > self.rust {
      println!("Highest marks in C.");
    } else if self.java > self.c && self.java > self.rust {
      println!("Highest marks in Java.");
    } else {
      println!("Highest marks in Rust.");
    }
  }
}

impl Student {
  fn new(name: String, c: i32, java: i32, rust: i32) -> Student {
    Student{ name, c, java, rust }
  }
}

fn section_eight_assignment() {
  let student1 = Student{
    name: "Bobby".to_string(),
    c: 10,
    java: 9,
    rust: 8,
  };
  println!("{:?}", student1);

  let student2 = Student{
    name: "Sammy".to_string(),
    c: 8,
    java: 10,
    rust: 9,
  };
  println!("{:?}", student1);

  let student3 = Student{
    name: "Mikey".to_string(),
    c: 9,
    java: 8,
    rust: 10,
  };
  println!("{:?}", student1);

  student1.highest();
  student2.highest();
  student3.highest();

  let student4 = Student::new("Mikey".to_string(), 9, 8, 10);
  println!("{:?}", student4);

  student4.highest();
}