//! # VBS Subsystem
//!

use anyhow::Result;
use instrument_ctl::Instrument;
use std::sync::Arc;

/// ## VBS Subsystem
/// 
/// Object for sending VBS commands and queries to the oscilloscope.
/// 
pub struct VbsSubsystem {
    client: Arc<Instrument>,
}

impl VbsSubsystem {
    /// ## Init
    /// 
    /// Initialize a VBS Subsystem object.
    /// 
    pub fn init(client: &Arc<Instrument>) -> VbsSubsystem {
        VbsSubsystem {
            client: client.clone(),
        }
    }
}

impl VbsSubsystem {
    /// ## VBS Command
    /// 
    /// Send a VBS command to the osciloscope.
    /// 
    pub fn vbs_command(&self, vbs_cmd: &str) -> Result<()> {
        let cmd = format!("VBS'{}';", vbs_cmd);
        self.client.command(&cmd)?;
        Ok(())
    }

    /// ## VBS Query
    /// 
    /// Send a VBS query and return the response as a string.
    /// 
    /// The 'Return=' part of a VBS query **MUST BE OMITTED** when using this
    /// method.
    /// 
    pub fn vbs_query(&self, vbs_cmd: &str) -> Result<String> {
        let cmd = format!("VBS'Return={}';", vbs_cmd);
        let resp = self.client.query(&cmd)?;
        Ok(resp)
    }

    /// ## VBS Query Raw
    /// 
    /// Send a VBS query and return the response as a vector of bytes.
    /// 
    /// The 'Return=' part of a VBS query **MUST BE OMITTED** when using this
    /// method.
    /// 
    pub fn vbs_query_raw(&self, vbs_cmd: &str) -> Result<Vec<u8>> {
        let cmd = format!("VBS'Return={}';", vbs_cmd);
        let resp = self.client.query_raw(&cmd)?;
        Ok(resp)
    }
}
