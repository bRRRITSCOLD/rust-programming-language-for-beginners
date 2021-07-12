extern crate section_10;
use section_10::*;

fn main() {
  communicator::client::connect();
  communicator::network::connect();
  communicator::network::server::connect();


  assignment::food::vegetable::lady_finger::taste();
  assignment::food::fruit::mango::taste();

  assignment::facebook::login::login(String::from("Username"), String::from("Password"));
  assignment::facebook::post::post(String::from("First Post!!!!"));
  assignment::facebook::logout::logout(String::from("Username"));
}