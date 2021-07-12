pub fn run() {
  println!("section 6 starting");

  section_five_ownsership_move_one();

  section_five_ownsership_move_two();

  section_five_reference_move_one();

  section_five_reference_move_two();

  section_five_reference_rules_one();

  section_five_reference_rules_two();

  section_five_reference_dangle_example();

  section_five_reference_slices_one();

  println!("section 6 finishing");
}

fn section_five_ownsership_move_one() {
  let s = String::from("hello");
  section_five_ownsership_move_take_one(s);
  // the below will now work because ownership was moved above and lost
  // after becoming out of scope when section_five_ownsership_move_take finishes
  // println!("{}", s); 
}

fn section_five_ownsership_move_take_one(s: String) {
  println!("{}", s);
}

fn section_five_ownsership_move_two() {
  let mut s = String::from("hello");
  s = section_five_ownsership_move_take_two(s);
  println!("{}", s); 
}

fn section_five_ownsership_move_take_two(s: String) -> String {
  println!("{}", s);
  s
}

fn section_five_reference_move_one() {
  let s = String::from("hello");
  section_five_reference_move_take_one(&s);
  // the below works because we use a reference and never lose ownership
  println!("{}", s);
}

// references as func params known as borrowing
fn section_five_reference_move_take_one(s: &String) {
  println!("{}", s);
  // wont work because the string/param being borrowed is not mutable
  // s.push_str("world");
}

fn section_five_reference_move_two() {
  let mut s = String::from("hello");
  section_five_reference_move_take_two(&mut s);
  // the below works because we use a reference and never lose ownership
  println!("{}", s);
}

// references as func params known as borrowing
fn section_five_reference_move_take_two(s: &mut String) {
  println!("{}", s);
  // the below works because we are borrowing a mutable param/var from outside the scope
  s.push_str("world");
}

fn section_five_reference_rules_one() {
  let mut s = String::from("hello");

  // not allowed - only one mutable ref per scope
  // let s1 = &mut s;
  // let s2 = &mut s;

  println!("{}", s);
  // println!("{}", s1);
  // println!("{}", s2);
}

fn section_five_reference_rules_two() {
  let mut s = String::from("hello");
  
  // not allowed - cannot create mut ref if you program
  // uses more than one immutable refs
  // let s1 = &mut s;
  // let s2 = &s;

  println!("{}", s);
  // println!("{}", s1);
  // println!("{}", s2);
}

fn section_five_reference_dangle_example() {
  // this is a dangle reference error, cannot return and store a value that
  // loses ownership/ref when descoped/returned
  // let a = section_five_reference_dangle();
  // println!("{}", a);
}

// this is a dangle reference error, cannot return and store a value that
// loses ownership/ref when descoped/returned
// fn section_five_reference_dangle() -> &String {
//   let d = String::from("hello");
//   &d
// }

fn section_five_reference_slices_one() {
  let a = String::from("hello world");

  let b1 = &a[0..5];
  println!("{:?}", b1);

  let b2 = &a[0..=5];
  println!("{:?}", b2);

  let b3 = &a[..5];
  println!("{:?}", b3);

  let b4 = &a[0..];
  println!("{:?}", b4);

  let b5 = &a[..];
  println!("{:?}", b5);
}