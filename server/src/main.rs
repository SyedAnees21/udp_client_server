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
        socket.send_to(&json_str.as_bytes(), &remote_adrr).expect("Unable to send data!");
        
    }
    
    
}


