use std::net::UdpSocket;
use std::io::Result;
use std::str;
use super::os_utils;

fn handle_msg(num_bytes: usize, src_addr: std::net::SocketAddr, msg: &[u8]) {
    let mut s = Vec::new();
    for c in 0..num_bytes {
        s.push(msg[c]);
    }

    println!("{}", str::from_utf8(&s)
        .expect("Failed to parse utf-8")
        .to_string());

    let out = os_utils::execute_command(str::from_utf8(&s)
        .expect("Failed to parse utf-8")
        .to_string());

    println!("Output >\n{}", out);
}

pub fn start_server() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8082")?;

    let mut buf = [0; 128];
    let (num_bytes, src_addr) = socket.recv_from(&mut buf)?;
    handle_msg(num_bytes, src_addr, &buf);

    Ok(())
}