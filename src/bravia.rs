use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::{IpAddr, TcpStream};

use log::{debug, error, info};

pub struct BraviaClient {
    ip: IpAddr,
}

// Port number used for Simple IP control.
const DEFAULT_PORT: u16 = 20060;

/**
 * Client for Bravia "Simple IP control" protocol
 * https://pro-bravia.sony.net/develop/integrate/ssip/command-definitions/index.html
 */
impl BraviaClient {
    pub fn new(ip: IpAddr) -> Self {
        BraviaClient { ip }
    }

    // setPictureMute
    pub fn picture_mute(&self, muted: bool) -> Result<(), Box<dyn Error>> {
        let cmd = if muted { "*SCPMUT0000000000000001\n" } else { "*SCPMUT0000000000000000\n" };
        self.send_command(cmd)
    }

    pub fn picture_mute_handled(&self, muted: bool) {
        if let Err(e) = self.picture_mute(muted) {
            error!("Error setting picture mute: {}", e)
        } else {
            info!("TV screen {}", if muted { "off" } else { "on" })
        }
    }

    fn send_command(&self, command: &str) -> Result<(), Box<dyn Error>> {
        let mut stream = TcpStream::connect((self.ip, DEFAULT_PORT))?;
        stream.write_all(command.as_bytes())?;

        let mut result = String::new();
        BufReader::new(stream).read_line(&mut result)?;
        debug!("TV response: {}", result.trim());
        Ok(())
    }
}
