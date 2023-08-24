//! ## Waveform Module
//!

use anyhow::{anyhow, Result};
use instrument_ctl::Instrument;
use std::sync::Arc;

use std::thread;
use std::time::Duration;


/// ## Waveform Subsystem
/// 
/// Object for controlling the saving of waveform data.
/// 
pub struct WaveformSubsystem {
    client: Arc<Instrument>,
}

impl WaveformSubsystem {
    /// ## Init
    /// 
    /// Initialize a Waveform Subsystem object.
    /// 
    pub fn init(client: &Arc<Instrument>) -> WaveformSubsystem {
        WaveformSubsystem {
            client: client.clone(),
        }
    }
}

impl WaveformSubsystem {
    /// The allowed traces.
    const TRACES: [&'static str; 9] = [
        "C1",
        "C2",
        "C3",
        "C4",
        "F1",
        "F2",
        "F3",
        "F4",
        "ALL_DISPLAYED",
    ];

    /// ## Set Autosave Mode Fill
    /// 
    /// Set the autosave mode to fill. This will begin as soon
    /// as it is set.
    /// 
    pub fn set_autosave_mode_fill(&self) -> Result<()> {
        self.client.command("STORE_SETUP AUTO,FILL")?;
        Ok(())
    }

    /// ## Set Autosave Mode Wrap
    /// 
    /// Set the autosave mode to wrap. This will begin as soon
    /// as it is set.
    /// 
    pub fn set_autosave_mode_wrap(&self) -> Result<()> {
        self.client.command("STORE_SETUP AUTO,WRAP")?;
        Ok(())
    }

    /// ## Set Autosave Mode Off
    /// 
    /// Set the autosave mode to off. This will stop files from
    /// being saved.
    /// 
    pub fn set_autosave_mode_off(&self) -> Result<()> {
        self.client.command("STORE_SETUP AUTO,OFF")?;
        Ok(())
    }

    /// ## Set Autosave Format ASCII
    /// 
    /// Set ASCII formatting for saved waveform files.
    /// 
    pub fn set_autosave_format_ascii(&self) -> Result<()> {
        self.client.command("STORE_SETUP FORMAT,ASCII")?;
        Ok(())
    }

    /// ## Set Autosave Format Binary
    /// 
    /// Set binary formatting for saved waveform files.
    /// 
    pub fn set_autosave_format_binary(&self) -> Result<()> {
        self.client.command("STORE_SETUP FORMAT,BINARY")?;
        Ok(())
    }

    /// ## Set Autosave Trace
    /// 
    /// Select a trace (functions or channels) from which data is saved to files.
    /// 
    pub fn set_autosave_trace(&self, trace: &str) -> Result<()> {
        // verify the trace
        if !Self::TRACES.contains(&trace) {
            return Err(anyhow!(
                "'{}' is not a valid trace\nvalid traces: {:?}",
                trace,
                Self::TRACES
            ));
        }

        // prepare the command
        let cmd = format!("STORE_SETUP {},HDD", trace);
        // execute the command
        self.client.command(&cmd)?;

        Ok(())
    }

    /// ## Set Autosave Path
    /// 
    /// Set the directory and the trace title to save.
    /// 
    pub fn set_autosave_path(&self, directory: &str, trace_title: &str) -> Result<()> {
        // replace the / with \
        let mut directory = directory.replace("/", "\\");
        // add a backslash to the end if it isn't already there
        if !directory.ends_with("\\") {
            directory.push('\\');
        }
        // add a backslash to the start if it isn't already there
        if !directory.starts_with("\\") {
            directory.insert(0, '\\');
        }

        let set_save_to_file: String = "VBS 'app.SaveRecall.Waveform.SaveTo=\"File\"'".into();
        let set_save_directory = format!("VBS 'app.SaveRecall.Waveform.WaveformDir=\"{}\"'", directory);
        let set_trace_title = format!("VBS 'app.SaveRecall.Waveform.TraceTitle=\"{}\"'", trace_title);

        self.client.command(&set_save_to_file)?;
        self.client.command(&set_save_directory)?;
        self.client.command(&set_trace_title)?;

        Ok(())
    }

    /// ## Wait Fill Complete
    /// 
    /// Block new commands from being send until the FILL autosave
    /// mode is switched to OFF or the scope cannot save more files
    /// to the directory.
    /// 
    pub fn wait_fill_complete(&self) -> Result<()> {
        loop {
            // get the current mode
            let resp: String= self.client.query("STST?")?;
            let mode = resp.split(",").collect::<Vec<&str>>()[3];

            // check that the mode is off
            if mode == "OFF"  { break; }

            // sleep to not overload cpu
            thread::sleep(Duration::from_millis(50));
        }

        Ok(())
    }
}
