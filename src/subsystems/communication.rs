//! # Communication Subsystem Module
//!

use anyhow::{anyhow, Result};
use instrument_ctl::Instrument;
use std::sync::Arc;

/// ## Communication Subsystem
///
/// Object used to control the communications subsystem.
///
pub struct CommunicationSubsystem {
    client: Arc<Instrument>,
}

impl CommunicationSubsystem {
    /// ## Init
    ///
    /// Initialize the object.
    ///
    pub fn init(client: &Arc<Instrument>) -> CommunicationSubsystem {
        // gather initial values
        CommunicationSubsystem {
            client: client.clone(),
        }
    }
}

impl CommunicationSubsystem {
    /// ## Get Remote Log
    ///
    /// Returns the remote log as a string. If `clear_log` is set to `true`,
    /// the remote log is cleared after being sent.
    ///
    pub fn read_remote_log(&self, clear_log: bool) -> Result<String> {
        let cmd = if clear_log {
            String::from("CHL?")
        } else {
            String::from("CHL? CLR")
        };

        let resp = self.client.query(&cmd)?;
        Ok(resp)
    }

    /// ## Get Log Level
    ///
    /// Return the log level in a human readable string format.
    ///
    pub fn get_log_level(&self) -> Result<String> {
        let resp = self.client.query("CHLP?")?;
        let resp: Vec<&str> = resp.split(",").collect();

        let level: String = match resp[0] {
            "OFF" => "off".into(),
            "FD" => "full dialog".into(),
            "EO" => "errors only".into(),
            _ => return Err(anyhow!("unrecognized log level value: {}", resp[0])),
        };

        Ok(level)
    }

    /// ## Set Log Level Off
    ///
    /// Sets the log level to OFF.
    ///
    pub fn set_log_level_off(&self) -> Result<()> {
        self.client.command("CHLP OFF,YES")?;
        Ok(())
    }

    /// ## Set Log Level Errors Only
    ///
    /// Sets the log level to EO.
    ///
    pub fn set_log_level_errors_only(&self) -> Result<()> {
        self.client.command("CHLP EO,YES")?;
        Ok(())
    }

    /// ## Set Log Level Full Dialog
    ///
    /// Sets the log level to FD.
    ///
    pub fn set_log_level_full_dialog(&self) -> Result<()> {
        self.client.command("CHLP FD,YES")?;
        Ok(())
    }
}
