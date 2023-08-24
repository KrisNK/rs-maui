//! Setup Subsystem
//!
//! Subsystem reponsible for saving and loading the panel setup of the oscilloscope.
//!

use anyhow::{anyhow, Result};
use instrument_ctl::Instrument;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::sync::Arc;

/// ## Setup Subsystem
/// 
/// Object for controlling saving and loading panel setups on the oscilloscope.
/// 
pub struct SetupSubsystem {
    client: Arc<Instrument>,
}

impl SetupSubsystem {
    /// ## Init
    /// 
    /// Intialize a Setup Subsystem object.
    /// 
    pub fn init(client: &Arc<Instrument>) -> SetupSubsystem {
        SetupSubsystem {
            client: client.clone(),
        }
    }
}

impl SetupSubsystem {
    /// ## Save Panel Setup
    ///
    /// Save the panel setup to the specified file path.
    ///
    pub fn save_panel_setup(&self, filepath: &str) -> Result<()> {
        // setup the filepath, making sure it includes the correct extension
        let mut filepath = filepath.to_owned();
        if !filepath.ends_with(".lss") {
            filepath += ".lss";
        }

        // open the file path, ensuring the file is new and the directory exists
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(filepath)?;

        // query the oscilloscope for the panel setup
        let resp = self.client.query("PNSU?")?;
        // trim the first 11 and last 8 bytes from the resp
        let setup = &resp[11..resp.len() - 8];

        // write the setup to the string
        file.write_all(setup.as_bytes())?;

        Ok(())
    }

    /// ## Load Panel Setup
    ///
    /// Send a panel setup from the controller to the device.
    ///
    pub fn load_panel_setup(&self, filepath: &str) -> Result<()> {
        // check that the file has the correct extension
        if !filepath.ends_with(".lss") {
            return Err(anyhow!(
                "file is not a panel setup file, it does not have a .lss extension"
            ));
        }

        // open the file and read the contents
        let mut file = OpenOptions::new().read(true).open(filepath)?;
        let mut setup: String = String::new();
        file.read_to_string(&mut setup)?;

        // Add the CRC string
        setup += "ffffffff";

        // get the number of bytes
        let setup_size: usize = setup.len();

        // add the byte size prefix
        setup.insert_str(0, &format!("#9{:0>9}", setup_size));

        // setup the command
        let cmd = format!("PNSU {}", setup);
        // run the command
        self.client.command(&cmd)?;

        Ok(())
    }
}
