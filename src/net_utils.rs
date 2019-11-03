use std::net::UdpSocket;
use std::str;
use std::error::Error;
use super::os_utils;

struct OtherInfo<'a> {
    socket: &'a UdpSocket,
    addr: std::net::SocketAddr,
}

fn send_output(other: OtherInfo, s: String) -> Result<(), Box<dyn Error>> {
    other.socket.send_to(s.as_bytes(), other.addr)?;
    Ok(())
}

fn handle_msg(other: OtherInfo, msg: &[u8], num_bytes: usize) -> Result<(), Box<dyn Error>> {
    let mut s = Vec::new();
    for c in 0..num_bytes {
        s.push(msg[c]);
    }
        
    let mut command = String::new();
    match str::from_utf8(&s) {
        Ok(com) => command = com.to_string(),
        Err(_) => (),
    };

    let out = match os_utils::execute_command(&command) {
        Ok(output) => output.to_string(),
        Err(_) => String::from("Invalid command, no result"),
    };
    
    // Send output to server
    send_output(other, out)?;

    Ok(())
}

pub fn start_server() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("127.0.0.1:8082")?;

    let mut buf = [0; 128];
    let (num_bytes, src_addr) = socket.recv_from(&mut buf)?;

    let other = OtherInfo {socket: &socket, addr: src_addr};
    handle_msg(other, &buf, num_bytes)?;

    Ok(())
}