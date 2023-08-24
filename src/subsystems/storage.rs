//! ## Storage Subsystem
//!
//! Subsystem for managing deleting and transfering files from storage.
//!

use anyhow::Result;
use instrument_ctl::Instrument;
use std::fs::OpenOptions;
use std::{
    io::{Read, Write},
    sync::Arc,
};

/// ## Storage Subsystem
/// 
/// Object for controlling the filesystem on the oscilloscope.
/// This includes:
/// - transferring files to and from the scope
/// - creating directories on the scope
/// - deleting files on the scope
/// 
pub struct StorageSubsystem {
    client: Arc<Instrument>,
}

impl StorageSubsystem {
    /// ## Init
    /// 
    /// Initialize a Storage System object.
    /// 
    pub fn init(client: &Arc<Instrument>) -> StorageSubsystem {
        StorageSubsystem {
            client: client.clone(),
        }
    }
}

impl StorageSubsystem {
    /// ### Delete File on Device
    ///
    /// Delete a specified file on the device.
    ///
    pub fn delete_file_on_device(&self, filepath: String) -> Result<()> {
        // make sure the filepath uses \ instead of /
        let device_filepath: String = filepath.replace("/", "\\");

        // setup the command
        let cmd: String = format!("DELETE_FILE DISK,HDD,FILE,'{}'", device_filepath);

        // execute the command
        self.client.command(&cmd)?;

        Ok(())
    }

    /// ### Transfer File to Device
    ///
    /// Transfer a file from the controller to the device.
    ///
    pub fn transfer_file_to_device(
        &self,
        device_filepath: String,
        controller_filepath: String,
    ) -> Result<()> {
        // get the data on the computer
        let mut file = OpenOptions::new().read(true).open(controller_filepath)?;
        let mut data: String = String::new();
        file.read_to_string(&mut data)?;

        // add the CRC and the bytes count
        data += "ffffffff";
        let data_size = data.len();
        data += &format!("#9{:0>9}", data_size);

        // make sure the device filepath has a DOS filepath
        let device_filepath = device_filepath.replace("/", "\\");

        // setup the command
        let cmd = format!("TRANSFER_FILE DISK,HDD,FILE,'{}',{}", device_filepath, data);
        // execute the command
        self.client.command(&cmd)?;

        Ok(())
    }

    /// ### Transfer File from Device
    ///
    /// Transfer a file from the device to the controller.
    ///
    pub fn transfer_file_from_device(
        &self,
        device_filepath: String,
        controller_filepath: String,
    ) -> Result<()> {
        // open the controller filepath
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(controller_filepath)?;

        // make sure the device filepath has a DOS filepath
        let device_filepath = device_filepath.replace("/", "\\");

        // setup the command
        let cmd = format!("TRANSFER_FILE? DISK,HDD,FILE,'{}'", device_filepath);
        // execute the command
        let data = self.client.query(&cmd)?;

        // remove the byte count and CRC string
        let data = data[11..data.len() - 8].to_owned();

        // save the data to the file
        file.write_all(&data.as_bytes())?;

        Ok(())
    }

    /// ## Create Directory on Device
    ///
    /// Create a directory on the device.
    ///
    pub fn create_directory_on_device(&self, directory: &str) -> Result<()> {
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

        // setup the command
        let set_dir_cmd: String = format!(
            "VBS 'app.SaveRecall.Utilities.Directory=\"{}\"'",
            directory
        );
        // let create_dir_cmd: String = "VBS 'app.SaveRecall.Utilities.CreateDir'".into();
        let create_dir_cmd: String = "VBS 'app.SaveRecall.Utilities.CreateDir';".into();

        // run the commands
        self.client.command(&set_dir_cmd)?;
        self.client.command(&create_dir_cmd)?;

        Ok(())
    }

    /// ## Delete All Files in Directory on Device
    ///
    /// Deletes all the files inside a directory on the device.
    ///
    pub fn delete_all_files_in_directory_on_device(&self, directory: &str) -> Result<()> {
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

        // setup the command
        let set_dir_cmd: String = format!(
            "VBS 'app.SaveRecall.Utilities.Directory=\"{}\"'",
            directory
        );
        let del_all_cmd: String = "VBS 'app.SaveRecall.Utilities.DeleteAll'".into();

        // run the commands
        self.client.command(&set_dir_cmd)?;
        self.client.command(&del_all_cmd)?;

        Ok(())
    }

    /// ## Get Screen Capture
    ///
    /// Get a screen shot of the current screen and save it to the specified file path.
    ///
    pub fn get_screen_capture(&self, filepath: &str) -> Result<()> {
        let mut filepath: String = filepath.into();
        if !filepath.ends_with(".jpeg") {
            filepath.push_str(".jpeg")
        }

        // open the file
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(filepath)?;

        // setup the screen dump 
        self.client.command("HCSU DEV,JPEG,FORMAT,LANDSCAPE,BCKG,BLACK,DEST,REMOTE,AREA,FULLSCREEN")?;
        // get the data?
        let data = self.client.query_raw("SCDP?")?;
        // write the data
        file.write_all(&data)?;

        Ok(())
    }
}
