# Rust MAUI

A pure Rust driver for Teledyne-Lecroy MAUI oscilloscopes.

## Motivation

The grand majority of Teledyne-Lecroy's programs are made for Windows. For cross-platform compatibility, Python libraries exist and could do what this library does. However, using the Python libraries lead to odd and unpredicable behaviour. Plus, Python is slow... relative to Rust.

Alongside this crate, `py-maui` will be developped by binding this library to Python with PyO3. This will make for easy adoption of the library.

## Roadmap

For now, this roadmap draws a rough outline of what needs to be done. The current goal is simply to replicate the commands available in MAUI's programming and automation manual. 

We will, as we already have, create abstraction that either split the listed methods in parts or combine them for ease of use.

**Version 0.1.x -- Baseline Release**
- The acquisition subsystem can be used to
    - Perform acquisitions
    - Set trigger modes (single, normal, auto, stop)
    - Set reference and sample clocks
    - Set time and volt division values
    - Set bandwidth limits and attenuations
- The communication subsystem can be used to
    - read the log
    - set the log level
- The VBS subsystem can be used to send VBS
    - commands
    - queries
- The storage subsystem can be used to
    - create directories on the device
    - transfer files to and from the device
    - delete files on the device
    - get a screen capture of the device
- The setup subsystem can be used to
    - save panel setups from device to controller
    - load panel setups from controller to device
- The waveform subsystem can 

**Version 0.2.x -- Fortification Update**
- The remaining acquisition methods will be implemented
- The status subsystem will be implemented
- Error handling will be properly implemented, taking advantage of the status subsystem
- Tests will be created for all methods. We will force the implementation of tests from this point onward.

**Version 0.3.x -- Display Update**
- Implemented subsystems:
    - Miscellaneous
    - Display

**Version 0.4.x -- Measurement Update**
- Implemented subsystems:
    - Cursor
    - Function

**Version 0.5.x -- Waveform Update**
- Re implementation of the waveform subsystem
    - Raw waveform data will be sent from the device to the controller
    - The controller (this program) will parse the data according to Lecroy's template


## Implemented Subsystems
- [ ] Status
- [ ] Acquisition
    - [x] ARM_ACQUISITION
    - [x] FORCE_TRIGGER
    - [x] STOP
    - [x] *TRG
    - [x] WAIT
    - [x] AUTO_SETUP
    - [x] ATTENUATION
    - [x] BANDWIDTH_LIMIT
    - [x] OFFSET
    - [x] TIME_DIV
    - [x] VOLT_DIV
    - [x] REFERENCE_CLOCK
    - [x] SAMPLE_CLOCK
    - [ ] COMBINE_CHANNELS
    - [ ] COUPLING
    - [ ] INTERLEAVED
    - [ ] SEQUENCE
    - [ ] MEMORY_SIZE
    - [ ] TRIG_COUPLING
    - [ ] TRIG_DELAY
    - [ ] TRIG_LEVEL
    - [x] TRIG_MODE
    - [ ] TRIG_PATTERN
    - [ ] TRIG_SELECT
    - [ ] TRIG_SLOPE
- [x] VBS
- [x] Communication
- [ ] Cursor
- [ ] Display
- [ ] Function
- [x] Hardcopy (part of storage)
- [ ] Miscellaneous
- [ ] Probes
- [x] Save/Recall Setup
    - [x] PANEL_SETUP
- [x] Storage
    - [x] DELETE_FILE
    - [x] TRANSFER_FILE
    - [x] VBS create directory
    - [x] VBS delete all files in directory
- [x] Waveform Transfer
    - [x] set modes
    - [x] set autosave paths
    - [x] set autosave formats
    - [x] wait until fill is complete
