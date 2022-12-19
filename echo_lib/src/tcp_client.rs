use std::{net::TcpStream, io::{Write, Read}, str::from_utf8, thread::sleep, time::Duration};

pub struct TcpClient {
    pub host: String,
    pub port: u32,
}

impl TcpClient {
    pub fn send(&self, msg: String) -> Result<(), Box<dyn std::error::Error>> {
        // Connect to the stream
        let mut stream = TcpStream::connect("0.0.0.0:3333")?;
    
        // Write the message to the stream
        stream.write(msg.as_bytes())?;
    
        // Read the response from the stream
        let mut data = [0 as u8; 14];
        stream.read_exact(&mut data)?;
    
        let txt = from_utf8(&data)?;
        println!("{}, size: {}", txt, data.len());
    
        // Sleep for 3 seconds
        sleep(Duration::from_secs(3));
    
        Ok(())
    }
    
    
    // pub fn send(&self, msg: String) {
    //     let result = TcpStream::connect("0.0.0.0:3333");
    //     match result {
    //         Ok(mut stream) => {
    //             stream.write(msg.as_bytes()).unwrap();
    //             let mut data = [0 as u8; 14];
    //             match stream.read_exact(&mut data) {
    //                 Ok(_) => {
    //                     let txt = from_utf8(&data);
    //                     println!("{}, size: {}", txt.unwrap(), data.len());
    //                 }
    //                 Err(e) => {
    //                     println!("{}", e)
    //                 }
    //             }
    //             sleep(Duration::from_secs(3));
    //         }
    //         Err(e) => {
    //             println!("{}", e);
    //         }
    //     }
    // }
}
