use std::net::*;
use std::thread::*;
use std::{env,str};


fn main() {
    let mcast_group: Ipv4Addr = "239.0.0.1".parse().expect("");
    let port = 6000_u16;
    let any = "0.0.0.0".parse().expect("");
    let mut buffer = [0u8; 1600];
    if env::args().count() > 1 {
        let socket = UdpSocket::bind((any, port))
            .expect("could not bind sockeet");
        socket.join_multicast_v4(&mcast_group, &any);
        socket.recv_from(&mut buffer).expect("failed to write to server");
        print!("{}", str::from_utf8(&buffer).expect(""));
    }else {
        let socket = UdpSocket::bind((any,0))
            .expect("could not create server ");
        socket.send_to("Hello world!".as_bytes(),&(mcast_group,port)).expect("");
    }
}
