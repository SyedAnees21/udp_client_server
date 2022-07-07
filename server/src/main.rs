// use std::{thread,time};
use std::net::UdpSocket;
use serde::{Serialize};
use serde_json;

#[derive(Serialize)]
struct Data {
    d1: String,
    d2: u8,
    d3: bool,
}

fn main() {

    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");
    let remote_adrr = "127.0.0.1:8000";
    //let mut buf: [u8; 5]= [1,2,3,4,5];

    let d = Data {
        d1: String::from("Anees"),
        d2: 35,
        d3: true
    };

    let json_str = serde_json::to_string(&d).unwrap();
    println!("Serialized structure for udp: {} of size {} ", json_str, json_str.len());

  

   // let  buf = String::new();

    loop {
        // let mut buf = [0u8; 1500];
        // let sock = socket.try_clone().expect("Failed to clone socket");

        // let (bytes, src_addr) = socket.peek_from(&mut buf).expect("Could not get the data");
        
        // match socket.recv_from(&mut buf) {
        //     Ok((_, src)) => {
        //         thread::spawn(move || {
        //             println!("Handling connection from {}", src);
                    
        //             println!("bytes {} , addr {}", bytes, src_addr);
        //             sock.send_to(&buf, &src).expect("Failed to send a response");
        //         });
        //     },
        //     Err(e) => {
        //         eprintln!("couldn't recieve a datagram: {}", e);
        //     }
        // }
        socket.send_to(&json_str.as_bytes(), &remote_adrr).expect("Unable to send data!");
        //delay();
    }
    
    
}


