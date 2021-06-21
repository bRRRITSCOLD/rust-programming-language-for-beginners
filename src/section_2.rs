use std::io;

fn section_two_even_odd() {
	let a=10;
	if a%2==0 {
		println!("Even");
	 }else {
		println!("Odd");
	}
}

fn section_two_greater() {
	let a=10;
	let b=30;
	
	if a>b {
		println!("A is Greater");
	}else {
		println!("B is Greater");
	}
}

fn section_two_friend() {
	let mut ch=String::new();
	println!("Are your friends coming ?");
	io::stdin().read_line(&mut ch).expect("Failed");
	ch=ch.trim().to_string();
	if ch=="y" {
		println!("Yeah! Going for Movie");
	}else {
		println!("Stay at home");
	}
}

fn section_two_social() {
	let mut name=String::new();
	let mut age=String::new();
	let mut ch=String::new();
	
	println!("Enter Name and Age");
	io::stdin().read_line(&mut name).expect("Failed");
	io::stdin().read_line(&mut age).expect("Failed");
	let age:i32=age.trim().parse().expect("Failed");
	
	println!("Do you want to create account");
	io::stdin().read_line(&mut ch).expect("Failed");
	ch=ch.trim().to_string();
	if ch=="y" {
		if age<10 {
			println!("Your age is less");
		}else {
			println!("Your Account is created");
			println!("Do you want to upload photo");
			ch.clear();
			io::stdin().read_line(&mut ch).expect("Failed");
			ch=ch.trim().to_string();
			println!("ch={}",ch);
			
			if ch=="y" {
				if age<13 {
					println!("You cannot upload photo");
				}else {
					println!("You can upload your photo");
				}
				
			}else {
				println!("Thanks for Visiting");
			}	
		}
	}
}

fn section_two_calc() {
	let mut o=String::new();
	let a=10;
	let b=2;
	println!("Choose operation to be performed");
	println!("1. +\n2. -\n3. *\n4. /");
	io::stdin().read_line(&mut o).expect("Failed");
	//o=o.trim().to_string();
	
	if o=="+" {
		println!("{}",a+b);
	}else if o=="-" {
		println!("{}",a-b);
	}else if o=="*" {
		println!("{}",a*b);

	}else if o=="/" {
		println!("{}",a/b);
	
	}else {
		println!("Wrong Choice");
	}
}

fn section_two_grade() {
	let mut per=String::new();
	println!("Enter your percentage");
	io::stdin().read_line(&mut per).expect("Fail");
	let per:i32=per.trim().parse().expect("Fail");
	if per>=90 {
		println!("A Grade");
	}else if per>=80  {
		println!("B Grade")
	}else if per>=70{
		println!("C Grade");
	}else if per>=60 {
		println!("D Grade");
	}else {
		println!("Fail");
	
	}
}

fn section_two_vowel() {
	let mut v=String::new();
	println!("Enter a Character");
	io::stdin().read_line(&mut v).expect("Failed");
	let v:char=v.trim().parse().expect("Failed");
	
	if v=='a' || v=='e' || v=='i' || v=='o' || v=='u' {
		println!("vowel");
	}else {
		println!("consonant");
	}
}

pub fn run() {
  println!("section 2 starting");

  section_two_even_odd();

  section_two_greater();
  
  section_two_friend();

  section_two_social();

  section_two_calc();

  section_two_grade();

  section_two_vowel();

  println!("section 2 finishing");
}