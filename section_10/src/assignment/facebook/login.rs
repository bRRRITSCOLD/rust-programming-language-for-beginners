pub fn login(user: String, password: String) -> i32 {
  if user == "Username" && password == "Password" {
    println!("Login successful");
    1
  } else {
    0
  }
}