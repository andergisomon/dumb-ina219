// Need to satisfy cargo to publish
// src/i2c_stub.rs
use std::io;

pub struct LinuxI2CDevice;
pub struct LinuxI2CError;

// Implement the same methods your code calls
impl LinuxI2CDevice {
    pub fn new(_path: &str, _addr: u16) -> Result<Self, LinuxI2CError> {
        println!("Warning: using stub I2C device on host");
        Ok(LinuxI2CDevice)
    }

    pub fn write(&mut self, _data: &[u8]) -> Result<(), LinuxI2CError> {
        println!("Stub write called");
        Ok(())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<(), LinuxI2CError> {
        println!("Stub read called, zeroing buffer");
        for b in buf.iter_mut() {
            *b = 0;
        }
        Ok(())
    }
}
