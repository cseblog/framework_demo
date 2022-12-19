use std::{
    io::{Read, Write},
    net::TcpListener,
    str::from_utf8,
    thread,
};

pub struct TcpServer {
    pub host: String,
    pub port: u32,
}

impl TcpServer {

    
    pub fn listen(&self) {                
        let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
        println!("TCP server on port 3333");

        for stream in listener.incoming() {
            //create a new thread
            let thread_handler = thread::spawn(move || {
                let mut data = [0 as u8; 17];
                match stream {
                    Ok(mut stream) => {
                        while match stream.read_exact(&mut data) {
                            Ok(size) => {
                                let txt = from_utf8(&data).unwrap();
                                println!("Received: {}, size:{}", txt, txt.len());
                                let mut reply = "Hi from Server";
                                stream.write(reply.as_bytes()).unwrap();
                                true
                            }
                            Err(e) => {println!("{}", e); false}
                        } {}
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            });
        }
    }
}
