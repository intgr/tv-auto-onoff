use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::{IpAddr, TcpStream};

use log::debug;

pub struct BraviaClient {
    conn: TcpStream,
}

// Port number used for Simple IP control.
const DEFAULT_PORT: u16 = 20060;

/**
 * Client for Bravia "Simple IP control" protocol
 * https://pro-bravia.sony.net/develop/integrate/ssip/command-definitions/index.html
 */
impl BraviaClient {
    pub fn new(ip: IpAddr) -> Result<Self, Box<dyn Error>> {
        let conn = TcpStream::connect((ip, DEFAULT_PORT))?;
        Ok(BraviaClient { conn })
    }

    // setPictureMute
    pub fn set_picture_mute(&mut self, muted: bool) -> Result<(), Box<dyn Error>> {
        let cmd = if muted { "*SCPMUT0000000000000001\n" } else { "*SCPMUT0000000000000000\n" };
        self.send_command(cmd)
    }

    fn send_command(&mut self, command: &str) -> Result<(), Box<dyn Error>> {
        debug!("Sending Bravia command: {}", command.trim());
        self.conn.write_all(command.as_bytes())?;

        let mut result = String::new();
        BufReader::new(&self.conn).read_line(&mut result)?;
        debug!("TV response: {}", result.trim());
        Ok(())
    }
}
