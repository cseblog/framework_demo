extern crate echo_stub_lib;
use echo_stub_lib::contract_stub::*;

struct EchoClient {}
impl EchoContractStub for EchoClient{}

fn main() {
    println!("Client service!");
    let client = EchoClient{};
    client.say_hello("hello world".to_string()); 
}
