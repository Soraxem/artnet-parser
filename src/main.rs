use std::net::Ipv4Addr;

mod artPoll;
mod artPollReply;


fn main() {
    println!("Hello, world!");
}


struct PortAddress {
    ip: u32,
    port: u16,
}

pub struct EstaManCode(u16);

pub struct OemCode(u16);



