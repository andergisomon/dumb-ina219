use dumb_ina219::{units::{CurrentUnit, Gettable, ResistanceUnit}, *};
use i2cdev::linux::LinuxI2CError;

const TARGET_ADDR: u8 = 0x44;

fn main() -> Result<(), LinuxI2CError> {
    let mut dev = Ina219::new(
        ResistanceUnit::milliohms(100.0),
        CurrentUnit::milliamps(1000.0),
        TARGET_ADDR)?;
    dev.init()?;
    let current_reading = dev.current()?;
    println!("{:?} mA", current_reading.get_val()*1000.0);
    Ok(())
}
