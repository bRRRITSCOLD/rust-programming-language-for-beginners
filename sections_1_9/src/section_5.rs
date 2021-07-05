pub fn run() {
  println!("section 5 starting");

  section_five_tuples();

  section_five_arrays();

  println!("section 5 finishing");
}

fn section_five_tuples() {
  let a: (i32,bool,f64) = (220,true,8.5);
  print_tuple(a)
}

fn print_tuple(x: (i32,bool,f64)) {
  println!("{:?}", x);
  println!("{}", x.1);
  let (a,y,z) = x;
  println!("{} {} {}", a, y, z);
}

fn section_five_arrays() {
  let a:[i32;5]=[3;5];
  print_array(a)
}

fn print_array(x: [i32;5]) {
  println!("{:?}", x);
  println!("{}", x.len());
  for n in x.iter() {
    println!("{}", n)
  }
}