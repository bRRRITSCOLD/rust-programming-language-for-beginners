pub fn run() {
  println!("section 4 starting");

  add_func_one();

  add_func_two(2, 3);

  let sum_one = add_func_three(2, 3);

  println!("{}", sum_one);

  let (sum_two, sub_one) = sub_add_func(4, 3);

  println!("{}, {}", sum_two, sub_one);

  let fact = factorial(5);

  println!("{}", fact);

  println!("section 4 finishing");
}

fn add_func_one() {
  println!("{}", 2+3)
}

fn add_func_two(a: i32, b: i32) {
  println!("{}", a+b)
}

fn add_func_three(a: i32, b: i32) -> i32 {
  a+b
}

fn sub_add_func(a: i32, b: i32) -> (i32, i32) {
  (a+b, a-b)
}

fn factorial(a: i32) -> i32 {
  let mut fact = 1;
  for n in 1 .. a + 1 {
    fact = fact * n;
  }
  fact
}