extern crate nix;
use std::net::*;
use std::{env,str};
use std::str::FromStr;
use std::collections::HashMap;
use std::time::*;
use std::os::unix::io::AsRawFd;
use std::{io, mem};
use std::thread::*;
use std::thread;
use nix::sys::socket;
use nix::sys::socket::sockopt::ReuseAddr;

fn connection_listener(){
    let mut cache : HashMap<SocketAddr, std::time::SystemTime> = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let socket = UdpSocket::bind("127.0.0.1:6000").expect("could not bind socket");

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
    let mut buffer = [0u8; 1600];
    socket::setsockopt(socket.as_raw_fd(), ReuseAddr, &true).expect("setsockopt failed");
    //socket.set_read_timeout(Option::from(Duration::from_secs(3)));
    loop {
        let (_, src_addr) = socket.recv_from(&mut buffer).expect("failed to read from server");
        cache.entry(src_addr).or_insert(std::time::SystemTime::now());
        println!("{}",cache.iter().filter(|(_,x)| { x.elapsed().unwrap()<Duration::from_secs(4) }).count());

        println!("{}",str::from_utf8(&mut buffer).unwrap());
    }
}

fn sender(){
    println!("start sending");
    let mcast_group: Ipv4Addr = "239.0.0.1".parse().expect("");
    let port = 6000_u16;
    let any:IpAddr = "0.0.0.0".parse().expect("");
    let mut buffer = [0u8; 1600];
    let socket = UdpSocket::bind((any,0))
        .expect("could not create server ");
    loop {
        match socket.send_to("Hello world!".as_bytes(),&(mcast_group, port)){
            Ok(size) => println!("send {}bytes", size),
            Err(e)   => println!("sending error"),
        }
        thread::sleep(Duration::from_millis(1000));
    }
}

fn main() {
    let _listener_handle = thread::Builder::new().name("listener".into()).spawn(connection_listener);
    thread::sleep(Duration::from_millis(100));
    let _sender_handle = thread::Builder::new().name("sender".into()).spawn(sender);
    _sender_handle.unwrap().join();
    _listener_handle.unwrap().join();
}
