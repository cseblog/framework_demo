extern crate echo_proxy_lib;
use echo_proxy_lib::contract_proxy::*;

struct EchoServer{}
impl EchoContractProxy for EchoServer{}

fn main() {
    println!("Hello, this Server");
    
    let server = EchoServer{};
    server.do_hello();

}
