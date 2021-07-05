extern crate section_10;

fn main() {
  section_10::communicator::client::connect();
  section_10::communicator::network::connect();
  section_10::communicator::network::server::connect();
}