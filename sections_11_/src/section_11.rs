use std::collections::HashMap;
use std::io;

fn section_eleven_vectors_one() {
	let mut v: Vec<i32> = Vec::new();
  v.push(20);
  v.push(30);
  v.push(40);
  println!("{:?}", v);
}

fn section_eleven_vectors_two() {
	let v = vec![1,2,3];
  println!("{:?}", v);
}

fn section_eleven_vectors_three() {
	let v = vec![1,2,3];
  println!("{:?}", v);
  let value = v[0];
  println!("{}", value);
  // this will not work because rust knows index
  // of 100 is out of bounds, due to len == 3
  // let value = v[100];
  let value = v.get(100);
  println!("{:?}", value);
}

fn section_eleven_vectors_four() {
	let v = vec![3,2,1];
  for value in &v {
    println!("{}", value);
  }
}

fn section_eleven_vectors_five() {
	let mut v = vec![3,2,1];
  for value in &mut v {
    *value*=2;
    println!("{}", value);
  }
}

#[derive(Debug)]
enum SpreadSheet {
  Integer(i32),
  Float(f64),
  Text(String),
}

fn section_eleven_vectors_six() {
	let row = vec![
    SpreadSheet::Integer(3),
    SpreadSheet::Float(3.2),
    SpreadSheet::Text(String::from("text")),
  ];
  println!("{:?}", row);
}

fn section_eleven_strings_one() {
  let a = 1;
  let mut s = a.to_string();
  s.push_str("Hello");
  s.push('0');
  println!("{}", s);
}

fn section_eleven_strings_two() {
  let s1 = String::from("Hello");
  let s2 = String::from("World");
  let s3 = s1 + &s2;
  println!("{}", s3);
}

fn section_eleven_strings_three() {
  let s1 = String::from("Hello");
  let s2 = String::from("World");
  let s3 = format!("{}-{}", s1, s2);
  println!("{}", s3);
  // format doesnt take ownership
  println!("{}", s1);
}

fn section_eleven_strings_four() {
  let s1 = String::from("Hello");
  let n1 = &s1[0..1];
  println!("{}", n1);
  
  for value in s1.chars() {
    println!("{}", value);
  }

  for value in "Hello".chars() {
    println!("{}", value);
  }
}

fn section_eleven_hash_maps_one() {
  let mut score = HashMap::new();
  score.insert("Blue", 20);
  score.insert("Red", 30);
  println!("{:?}", score);
}

fn section_eleven_hash_maps_two() {
  let teams = vec!["Blue", "Red"];
  let scores = vec![10, 20];
  let collected_hash_map: HashMap<_,_> = teams.iter().zip(scores.iter()).collect();
  println!("{:?}", collected_hash_map);
}

fn section_eleven_hash_maps_three() {
  let mut scores = HashMap::new();
  scores.insert("Blue", 20);
  scores.insert("Red", 30);
  let blue_team = scores.get("Blue");
  println!("{:?}", blue_team);
  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }
}

fn section_eleven_hash_maps_four() {
  let mut scores = HashMap::new();
  scores.insert("Blue", 10);
  scores.entry("Blue").or_insert(20);
  scores.entry("Red").or_insert(30);
  println!("{:?}", scores);
}

fn section_eleven_assignment() {
  let mut n=String::new();
  let mut name=String::new();
  let mut no=String::new();
  let mut v_name:Vec<String>=Vec::new();
  let mut v_no:Vec<String>=Vec::new();
  let mut c=1;
  let mut choice=String::new();

  println!("How many contacts you want to store");
  io::stdin().read_line(&mut n).expect("Fail");
  let n:u32=n.trim().parse().expect("Fail");

  while c<=n{
    name.clear();
    no.clear();
    println!("Enter name than no.");
    
    io::stdin().read_line(&mut name).expect("Fail");
    let name:String=name.trim().parse().expect("Failed");
    
    io::stdin().read_line(&mut no).expect("Fail");
    let no:String=no.trim().parse().expect("Failed");
    
    v_name.push(name);
    v_no.push(no);
    c+=1;

  }
  println!("{:?} {:?}",v_name,v_no);

  let contact:HashMap<&String,&String>=v_name.iter().zip(v_no.iter()).collect();

  println!("{:?}",contact);

  println!("Which Name to Search");
  io::stdin().read_line(&mut choice).expect("Fail");
  let choice:String=choice.trim().parse().expect("Fail");

  fn search(contact:HashMap<&String,&String>,choice:&String){
    for (k,v) in contact {
      if k==choice {
        println!("Contact: {} {} ",k,v);
      }
    }
  }
  search(contact,&choice);
}

pub fn run() {
  println!("section 11 starting");

  section_eleven_vectors_one();

  section_eleven_vectors_two();

  section_eleven_vectors_three();

  section_eleven_vectors_four();

  section_eleven_vectors_five();

  section_eleven_vectors_six();

  section_eleven_strings_one();

  section_eleven_strings_two();

  section_eleven_strings_three();

  section_eleven_strings_four();

  section_eleven_hash_maps_one();

  section_eleven_hash_maps_two();

  section_eleven_hash_maps_three();

  section_eleven_hash_maps_four();

  section_eleven_assignment();

  println!("section 11 finishing");
}