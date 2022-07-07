use std::net::UdpSocket;
use serde::{Deserialize,Serialize};
use serde_json;
use std::str;


#[derive(Serialize,Deserialize,Debug)]
struct Data {
    d1: String,
    d2: u8,
    d3: bool,
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8000").expect("Could not bind client socket");
    socket.connect("127.0.0.1:8888").expect("Could not connect to server");

    let mut buf= [0; 40];
    
    let a = socket.recv(&mut buf).expect("Could not get the datagram");
    let json_str = str::from_utf8(&mut buf).expect("unable to parse");
    
    
    let j_str = json_str.trim_matches('\0').to_string();
    println!("Message:{} Message_size {}", j_str, a);
   
    let d:Data = serde_json::from_str(j_str.as_str()).unwrap();
    println!("{:?}", d.d1);
}
 