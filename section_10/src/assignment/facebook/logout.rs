pub fn logout(user: String) -> i32 {
  if user == "Username" {
    println!("Logout successful");
    1
  } else {
    0
  }
}