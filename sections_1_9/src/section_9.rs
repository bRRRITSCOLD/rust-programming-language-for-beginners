pub fn run() {
  println!("section 9 starting");

  section_nine_enum_one();

  section_nine_enum_two();

  section_nine_enum_three();

  section_nine_enum_four();

  section_nine_enum_five();

  section_nine_enum_six();

  section_nine_enum_seven();

  section_nine_enum_eight();

  println!("section 9 finishing");
}

#[derive(Debug)]
enum IPAddressKind {
  V4,
  V6,
}

#[derive(Debug)]
struct IPAddress {
  kind: IPAddressKind,
  address: String,
}

fn route(ip_kind: IPAddressKind) {
  println!("{:?}", ip_kind);
}

fn section_nine_enum_one() {
  let v4 = IPAddressKind::V4;
  println!("{:?}", v4);

  let v6 = IPAddressKind::V6;
  println!("{:?}", v6);

  route(IPAddressKind::V4);
  route(IPAddressKind::V6);
}

#[derive(Debug)]
enum Fruits {
  Apple = 0,
  Mango = 10,
  Watermelon = 20,
}

#[derive(Debug)]
enum IPAddressOne {
  V4(String),
  V6(String),
}

#[derive(Debug)]
enum IPAddressTwo {
  V4(u8, u8, u8, u8),
  V6(String),
}

fn section_nine_enum_two() {
  let ip_address1 = IPAddress{
    kind: IPAddressKind::V4,
    address: String::from("127.0.0.1")
  };
  println!("{:?}", ip_address1);
  
  let ip_address2 = IPAddress{
    kind: IPAddressKind::V6,
    address: String::from("::1")
  };
  println!("{:?}", ip_address2);

  let fruit1 = Fruits::Apple;
  println!("{:?}", fruit1 as i32);

  let fruit2 = Fruits::Mango;
  println!("{:?}", fruit2 as i32);

  let fruit3 = Fruits::Watermelon;
  println!("{:?}", fruit3 as i32);

  let ip_address_one_1 = IPAddressOne::V4(String::from("127.0.0.1"));
  println!("{:?}", ip_address_one_1);

  let ip_address_one_2 = IPAddressOne::V6(String::from("::1"));
  println!("{:?}", ip_address_one_2);

  let ip_address_two_1 = IPAddressTwo::V4(127, 0, 0, 1);
  println!("{:?}", ip_address_two_1);

  let ip_address_two_2 = IPAddressTwo::V6(String::from("::1"));
  println!("{:?}", ip_address_two_2);
}

fn section_nine_enum_three() {
  let some1 = Some(5);
  println!("{:?}", some1);

  let some2 = Some(10);
  println!("{:?}", some2);
  
  let some3: Option<i32> = None;
  println!("{:?}", some3);
  
  let some4: Option<&str> = None;
  println!("{:?}", some4);

  // cant do this, however you can with
  // generics (or something like it or close to it)
  // let some5 = Some(5) + 5;
  // println!("{:?}", some5);
}

#[derive(Debug)]
enum CoinOne {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents_one(c: CoinOne) -> u32 {
  match c {
    CoinOne::Penny => {
      println!("Penny");
      1
    },
    CoinOne::Nickel => {
      println!("Nickel");
      5
    },
    CoinOne::Dime => {
      println!("Dime");
      10
    },
    CoinOne::Quarter => {
      println!("Quarter");
      25
    },
  }
}

fn section_nine_enum_four() {
  let value_in_cents_one1 = value_in_cents_one(CoinOne::Penny);
  println!("{:?}", value_in_cents_one1);

  let value_in_cents_one2 = value_in_cents_one(CoinOne::Nickel);
  println!("{:?}", value_in_cents_one2);
  
  let value_in_cents_one3 = value_in_cents_one(CoinOne::Dime);
  println!("{:?}", value_in_cents_one3);
  
  let value_in_cents_one4 = value_in_cents_one(CoinOne::Quarter);
  println!("{:?}", value_in_cents_one4);
}

#[derive(Debug)]
enum UsStateOne {
  Alaska,
  Arizona,
  Alabama,
  Delaware
}

#[derive(Debug)]
enum CoinTwo {
  Penny,
  Nickel,
  Dime,
  Quarter(UsStateOne),
}

fn value_in_cents_two(c: CoinTwo) -> u32 {
  match c {
    CoinTwo::Penny => {
      println!("Penny");
      1
    },
    CoinTwo::Nickel => {
      println!("Nickel");
      5
    },
    CoinTwo::Dime => {
      println!("Dime");
      10
    },
    CoinTwo::Quarter(state) => {
      println!("{:?} Quarter", state);
      25
    },
  }
}

fn section_nine_enum_five() {
  let value_in_cents_two1 = value_in_cents_two(CoinTwo::Penny);
  println!("{:?}", value_in_cents_two1);

  let value_in_cents_two2 = value_in_cents_two(CoinTwo::Nickel);
  println!("{:?}", value_in_cents_two2);
  
  let value_in_cents_two3 = value_in_cents_two(CoinTwo::Dime);
  println!("{:?}", value_in_cents_two3);

  let value_in_cents_two4 = value_in_cents_two(CoinTwo::Quarter(UsStateOne::Alabama));
  println!("{:?}", value_in_cents_two4);

  let value_in_cents_two5 = value_in_cents_two(CoinTwo::Quarter(UsStateOne::Alaska));
  println!("{:?}", value_in_cents_two5);

  let value_in_cents_two6 = value_in_cents_two(CoinTwo::Quarter(UsStateOne::Arizona));
  println!("{:?}", value_in_cents_two6);

  let value_in_cents_two7 = value_in_cents_two(CoinTwo::Quarter(UsStateOne::Delaware));
  println!("{:?}", value_in_cents_two7);
}

fn plus_one_one(x: Option<i32>) -> Option<i32> {
  match x {
    None=>None,
    Some(i)=>Some(i+1),
  }
}

fn section_nine_enum_six() {
  let plus_one_one1 = plus_one_one(Some(5));
  println!("{:?}", plus_one_one1);

  let plus_one_one2 = plus_one_one(None);
  println!("{:?}", plus_one_one2);
}

fn plus_one_two(x: Option<i32>) -> Option<i32> {
  match x {
    Some(i)=>Some(i+1),
    _=>None,
  }
}

fn section_nine_enum_seven() {
  let plus_one_two1 = plus_one_two(Some(5));
  println!("{:?}", plus_one_two1);

  let plus_one_two2 = plus_one_two(None);
  println!("{:?}", plus_one_two2);
}

fn if_let_one(x: u32) {
  let some = Some(x);

  if let Some(3) = some {
    println!("three");
  } else if let Some(4) = some {
    println!("four");
  } else {
    println!("some other number");
  }
}

fn section_nine_enum_eight() {
  if_let_one(3);
  if_let_one(4);
  if_let_one(5);
}