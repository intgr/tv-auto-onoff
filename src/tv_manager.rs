use std::net::IpAddr;

use log::{error, info};

use crate::bravia::BraviaClient;
use crate::util::BoxError;

pub struct TvManager {
    ip: IpAddr,
}

impl TvManager {
    pub fn new(ip: IpAddr) -> Self {
        TvManager { ip }
    }

    fn connect(&self) -> Result<BraviaClient, BoxError> {
        BraviaClient::new(self.ip)
    }

    fn turn_on_internal(&self) -> Result<(), BoxError> {
        let mut bravia = self.connect()?;
        // This ordering is significant, opposite order sometimes results in blank screen.
        bravia.set_picture_mute(false)?;
        bravia.set_power_status(true)?;
        Ok(())
    }

    pub fn turn_on(&self) {
        if let Err(e) = self.turn_on_internal() {
            error!("Error turning on TV: {e}");
        } else {
            info!("TV turned on");
        }
    }

    fn turn_off_internal(&self) -> Result<(), BoxError> {
        let mut bravia = self.connect()?;
        bravia.set_picture_mute(true)?;
        Ok(())
    }

    pub fn turn_off(&self) {
        if let Err(e) = self.turn_off_internal() {
            error!("Error turning off TV: {e}");
        } else {
            info!("TV turned off");
        }
    }

    fn keepalive_internal(&self) -> Result<(), BoxError> {
        let mut bravia = self.connect()?;
        // Calling setPictureMute resets the sleep timer. But we need to know current status to avoid changing it.
        // XXX Apparenty getPictureMute can give incorrect results if we call it too quickly.
        let muted = bravia.get_picture_mute()?;
        bravia.set_picture_mute(muted)?;
        Ok(())
    }

    /// Reset TV sleep timer (called periodically)
    pub fn keepalive(&self) {
        if let Err(e) = self.keepalive_internal() {
            error!("Error sending keep-alive ping: {e}");
        } else {
            info!("Keep-alive sent");
        }
    }
}
