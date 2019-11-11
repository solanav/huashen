use std::net::{UdpSocket, SocketAddr};

// Public
pub mod active;
pub mod reactive;

pub struct OtherInfo<'a> {
    socket: &'a UdpSocket,
    addr: SocketAddr,
}