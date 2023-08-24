//! ## Utils Module
//! 
//! A set of utility functions used across the crate.
//! 

use instrument_ctl::Instrument;
use std::sync::Arc;
use anyhow::Result;

use std::thread;
use std::time::Duration;

/// ## Operation Complete
/// 
/// Blocks sending new commands until the operation is complete.
/// 
pub fn wait_operation_complete(client: &Arc<Instrument>) -> Result<()> {
    // Just loop until the operation is complete. In reality, the loop isn't required.
    loop {
        match client.query("*OPC?")?.as_str() {
            "0" => thread::sleep(Duration::from_micros(10)),
            "1" => break,
            _ => panic!("invalid response from *OPC?"),
        };
    }
    Ok(())
}