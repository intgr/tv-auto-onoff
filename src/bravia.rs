use std::io::{BufRead, BufReader, Write};
use std::net::{IpAddr, TcpStream};

use log::{debug, trace};
use simple_error::bail;

use crate::util::BoxError;

#[allow(clippy::module_name_repetitions)]
pub struct BraviaClient {
    conn: TcpStream,
}

// Port number used for Simple IP control.
const DEFAULT_PORT: u16 = 20060;

/// Client for Bravia "Simple IP control" protocol
/// <https://pro-bravia.sony.net/develop/integrate/ssip/command-definitions/index.html>
impl BraviaClient {
    pub fn new(ip: IpAddr) -> Result<Self, BoxError> {
        let conn = TcpStream::connect((ip, DEFAULT_PORT))?;
        Ok(BraviaClient { conn })
    }

    // setPictureMute
    pub fn set_picture_mute(&mut self, muted: bool) -> Result<(), BoxError> {
        let cmd = if muted { "*SCPMUT0000000000000001\n" } else { "*SCPMUT0000000000000000\n" };
        self.send_command(cmd)
    }

    // setPowerStatus
    pub fn set_power_status(&mut self, active: bool) -> Result<(), BoxError> {
        let cmd = if active { "*SCPOWR0000000000000001\n" } else { "*SCPOWR0000000000000000\n" };
        self.send_command(cmd)
    }

    // getPictureMute
    pub fn get_picture_mute(&mut self) -> Result<bool, BoxError> {
        let result = self.transact("*SEPMUT################\n")?;
        if result == "*SAPMUT0000000000000000" {
            return Ok(false);
        } else if result == "*SAPMUT0000000000000001" {
            return Ok(true);
        }
        bail!("Protocol error: Unexpected getPictureMute response: {result}");
    }

    fn transact(&mut self, command: &str) -> Result<String, BoxError> {
        trace!("Sending Bravia command: {}", command.trim_end());
        self.conn.write_all(command.as_bytes())?;

        let mut reader = BufReader::new(&self.conn);
        loop {
            let mut result = String::new();
            // XXX no timeout
            reader.read_line(&mut result)?;
            let maybe_slice = result.strip_suffix('\n');

            if maybe_slice.is_none() {
                // EOF hit without newline.
                bail!(
                    "Protocol error: Connection closed without complete response (got {} bytes)",
                    result.len()
                );
            }
            let slice = maybe_slice.unwrap();
            if slice.len() != 23 {
                bail!("Protocol error: Unexpected response length {} bytes", slice.len());
            }
            if result.starts_with("*SN") {
                // Ignore async notifications, they're not the reply to the command. Loop back again.
                debug!("Bravia asynchronous notification: {slice}");
            } else if result.starts_with("*SA") {
                // We assume this is reply to the command, without checking deeper.
                trace!("Received Bravia response: {slice}");
                return Ok(slice.to_string());
            } else {
                bail!("Protocol error: Unrecognized response packet: {slice}");
            }
        }
    }

    fn send_command(&mut self, command: &str) -> Result<(), BoxError> {
        let result = self.transact(command)?;
        if result.ends_with("FFFFFFFFFFFFFFFF") {
            bail!("Protocol error: Command {} error result: {result}", command.trim_end());
        }
        Ok(())
    }
}
