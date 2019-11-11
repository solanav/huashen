use super::OtherInfo;
use std::error::Error;

pub fn send_output(other: OtherInfo, s: String) -> Result<(), Box<dyn Error>> {
    other.socket.send_to(s.as_bytes(), other.addr)?;
    Ok(())
}