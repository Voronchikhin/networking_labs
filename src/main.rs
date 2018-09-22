extern crate nix;
extern crate core;

use std::net::*;
use std::{env,str};
use std::str::FromStr;
use std::collections::HashMap;
use std::time::*;
//use std::os::unix::io::AsRawFd;
use std::{io, mem};
use std::thread::*;
use std::thread;
use std::ops::Add;
//use nix::sys::socket;
//use nix::sys::socket::sockopt::ReusePort;
use core::borrow::BorrowMut;

fn sender(){
    println!("start sending");
    let mcast_group: Ipv4Addr = "239.0.0.1".parse().expect("");
    let port = 0_u16;
    let any:IpAddr = "0.0.0.0".parse().expect("");
    let buffer = [0u8; 1600];
    let socket = UdpSocket::bind((any,0))
        .expect("could not create server ");
    println!("{}",socket.multicast_loop_v4().unwrap());
    loop {
        match socket.send_to("Hello world!".as_bytes(),&(mcast_group, port)){
            Ok(size) => println!("send {}bytes", size),
            Err(e)   => println!("sending error"),
        }
        thread::sleep(Duration::from_millis(1000));
    }
}

fn connection_listener(){
    let args: Vec<String> = env::args().collect();
    let socket = UdpSocket::bind("0.0.0.0:0").expect("could not bind socket");

    socket.set_multicast_loop_v4(true).expect("could not use multicast");
    println!("{:?}",socket);
    let mcast_group: IpAddr =  match args.get(1){
        Some(str) => str.parse(),
        None => "239.0.0.1".parse(),
    }.expect("wrong address");
    match mcast_group {
        IpAddr::V4(ipv4) => socket.join_multicast_v4(&ipv4,&Ipv4Addr::new(0,0,0,0)),
        IpAddr::V6(ipv6) => socket.join_multicast_v6(&ipv6,0),
    };
    let mut buffer = [0u8; 12];
    socket.set_read_timeout(Option::from(Duration::from_secs(3)));
    let mut cache :HashMap<SocketAddr,SystemTime> = HashMap::new();
    loop {
        let (_, src_addr) = match socket.recv_from(&mut buffer){
            Ok( (i,addr) ) => (i, addr),
            Err(_) => (0, SocketAddr::from_str("0.0.0.0:0").unwrap()),
        };

        cache.insert(src_addr,SystemTime::now());

        println!("{}",cache.iter_mut()
            .filter(|(_,x)|
                {SystemTime::now().duration_since(**x).unwrap() < Duration::from_secs(2)}).count()
        );

        println!("{}",str::from_utf8(&mut buffer).unwrap());
    }
}

fn main() {
    let _listener_handle = thread::Builder::new().name("listener".into()).spawn(connection_listener);
    thread::sleep(Duration::from_millis(100));
    let _sender_handle = thread::Builder::new().name("sender".into()).spawn(sender);
    _sender_handle.unwrap().join();
    _listener_handle.unwrap().join();
}
