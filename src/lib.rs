//! # Rust MAUI
//!
//! A pure Rust driver for Teledyne-Lecroy MAUI oscilloscopes.
//!

use anyhow::Result;
use instrument_ctl::Instrument;
use std::{sync::Arc, time::Duration};

mod subsystems {
    pub mod acquisition;
    pub mod communication;
    pub mod setup;
    pub mod storage;
    pub mod waveform;
    pub mod vbs;
}

mod utils;

use subsystems::{
    acquisition::AcquisitionSubsystem, communication::CommunicationSubsystem,
    setup::SetupSubsystem, storage::StorageSubsystem, vbs::VbsSubsystem,
    waveform::WaveformSubsystem,
};

/// ## MAUI Oscilloscope
///
/// Object through which communication with the oscilloscope is done.
///
pub struct MauiOscilloscope {
    client: Arc<Instrument>,
    pub communication: CommunicationSubsystem,
    pub vbs: VbsSubsystem,
    pub acquisition: AcquisitionSubsystem,
    pub setup: SetupSubsystem,
    pub storage: StorageSubsystem,
    pub waveform: WaveformSubsystem,
}

impl MauiOscilloscope {
    /// ## Connect
    ///
    /// Connect and initialize the device.
    ///
    pub fn connect(visa_address: &str) -> Result<MauiOscilloscope> {
        // Connect
        let client = Arc::new(Instrument::connect(visa_address)?);

        // Setup the status
        // let status = Arc::new(Status::init(&client));

        // Set the COMM HEADERS off
        client.command("CHDR OFF")?;

        // Enable standard events to be reflected in the status byte
        let mask: u8 = 0b1111_1111;
        let cmd = format!("*ESE {}", mask);
        client.command(&cmd)?;

        // Enable internal state changes to be reflected in the status byte
        let mask: u16 = 0b0111_1111_1101_1111;
        let cmd = format!("INE {}", mask);
        client.command(&cmd)?;

        // Setup the subsystems
        let communication = CommunicationSubsystem::init(&client);
        let vbs = VbsSubsystem::init(&client);
        let acquisition = AcquisitionSubsystem::init(&client);
        let setup = SetupSubsystem::init(&client);
        let storage = StorageSubsystem::init(&client);
        let waveform = WaveformSubsystem::init(&client);

        Ok(MauiOscilloscope {
            client,
            communication,
            vbs,
            acquisition,
            setup,
            storage,
            waveform,
        })
    }

    /// ## Set Timeout
    ///
    /// Set a new timeout duration for the oscilloscope connection.
    ///
    pub fn set_timeout(&self, duration: Duration) {
        self.client.set_timeout(duration);
    }

    /// ## Command
    ///
    /// Send a command to the oscilloscope.
    ///
    pub fn command(&self, cmd: &str) -> Result<()> {
        self.client.command(cmd)?;
        Ok(())
    }

    /// ## Query
    ///
    /// Send a command to the oscilloscope and return the response as a string.
    ///
    pub fn query(&self, cmd: &str) -> Result<String> {
        let resp = self.client.query(cmd)?;
        Ok(resp)
    }

    /// ## Query Raw
    ///
    /// Send a command to the oscilloscope and return the response as a vector of bytes.
    ///
    pub fn query_raw(&self, cmd: &str) -> Result<Vec<u8>> {
        let resp = self.client.query_raw(cmd)?;
        Ok(resp)
    }
}
