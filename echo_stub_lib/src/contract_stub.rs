//This is an example a service lib
use std::{
    io::{Read, Write},
    net::TcpStream,
    str::from_utf8,
    thread::sleep,
    time::Duration,
};

use echo_lib::tcp_client::*;

pub trait EchoContractStub {
    
    //Client service say hello
    fn say_hello(&self, msg: String) {
        let tcp_client = TcpClient {
            host: "127.0.0.0".to_string(),
            port: 9000,
        };
        tcp_client.send("hello world".to_string());
    }
}
