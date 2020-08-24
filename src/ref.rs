//! Thanks egmkang wang.
//! The fn get() which is from his [local_addr](https://github.com/egmkang/local_ipaddress) 
//! The License is MIT
//! A crate for get local ip address,
//! without using `ifconfig` or scanning `network interface`

/*
use pnet::datalink;

fn main() {
    for iface in datalink::interfaces() {
        println!("{:?}", iface.ips);
    }
}
*/
use std::net::UdpSocket;

/// get the local ip address, return an `Option<String>`. when it fail, return `None`.
pub fn get() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}

//use local_ipaddress;

fn main() {
    if let Some(x) = get() {
        println!("The ip address is {}", x);
    } else {
        println!("None");
    }

    if true == false {
        println!("oh no");
    } else if "something" == "other thing" {
        println!("oh dear");
    } else if let Some(200) = "blarg".parse::<i32>().ok() {
        println!("uh oh");
    } else {
        println!("phew, nothing's broken");
    }
}