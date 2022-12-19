use echo_lib::tcp_server::TcpServer;
use std::{
    io::{Read, Write},
    net::TcpListener,
    str::from_utf8,
    thread,
};
pub trait EchoContractProxy {
    // Server service will implement this function
    fn do_hello(&self) {
        let tcp_server = TcpServer {
            host: String::from("0.0.0.0"),
            port: 9000,
        };
        tcp_server.listen();
    }
}
