// Need to satisfy cargo to publish
pub struct LinuxI2CDevice;
pub struct LinuxI2CError;

impl LinuxI2CDevice {
    pub fn new(_path: &str, _addr: u16) -> Result<Self, LinuxI2CError> {
        Err(LinuxI2CError)
    }
}
