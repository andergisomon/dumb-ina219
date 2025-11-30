extern crate i2cdev;
use i2cdev::core::*;
#[cfg(any(target_os = "linux"))]
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use std::{thread::sleep, time::Duration};

#[cfg(not(any(target_os = "linux")))]
mod stub;
#[cfg(not(any(target_os = "linux")))]
use crate::stub::{LinuxI2CDevice, LinuxI2CError};

pub mod units;
use crate::units::*;

enum RegAddrs {
    Cfg          = 0x00,
    ShuntVoltage = 0x01,
    BusVoltage   = 0x02,
    Power        = 0x03,
    Current      = 0x04,
    Calibration  = 0x05,
}

pub struct Ina219 {
    shunt_resistance: ResistanceUnit,
    max_expected_current: CurrentUnit,
    current_lsb: CurrentUnit,
    power_lsb: PowerUnit,
    i2c_address: u8,
    device: LinuxI2CDevice,
}

impl Ina219 {
    /// You **must** init after creating a new INA219 instance.
    pub fn new(
        shunt_resistance: ResistanceUnit,
        max_expected_current: CurrentUnit,
        address: u8,
    ) -> Result<Self, LinuxI2CError> {
        Ok(
            Self {
                shunt_resistance: shunt_resistance,
                max_expected_current: max_expected_current,
                current_lsb: CurrentUnit::milliamps(0.0),
                power_lsb: PowerUnit::milliwatts(0.0),
                i2c_address: address,
                device: LinuxI2CDevice::new("/dev/i2c-1", address as u16)?
            }
        )
    }

    /// This **must** be called before calling methods to read voltage/current/power measurements.
    pub fn init(&mut self) -> Result<(), LinuxI2CError> {
        sleep(Duration::from_millis(650)); // slight delay after bus init
        let current_lsb_val = self.max_expected_current.get_val() / 32768.0;
        self.current_lsb.set_val(current_lsb_val);

        let power_lsb_val = 20.0 * current_lsb_val;
        self.power_lsb.set_val(power_lsb_val);
        self.device.write(&[RegAddrs::Cfg as u8, 0x1F as u8, 0xFF as u8])?; // Default config
        Ok(())
    }

    fn calibrate(&mut self) -> Result<(), LinuxI2CError> {
        let cal = (0.04096 / (self.current_lsb.get_val() * self.shunt_resistance.get_val())).trunc() as u16;
        self.device.write(
            &[RegAddrs::Calibration as u8, (cal >> 8) as u8, cal as u8]
        )?;
        Ok(())
    }

    pub fn shunt_voltage(&mut self) -> Result<VoltageUnit, LinuxI2CError> {
        let mut buf: [u8; 2] = [0; 2];
        self.calibrate()?;
        self.device.write(&[RegAddrs::ShuntVoltage as u8])?;
        self.device.read(&mut buf)?;
        let val = (i16::from_be_bytes(buf)) as f64;
        Ok(VoltageUnit::volts(val*0.00001)) // 10uV step size: 10uV * (1V / 10^6 uV)
    }

    pub fn bus_voltage(&mut self) -> Result<VoltageUnit, LinuxI2CError> {
        let mut buf: [u8; 2] = [0; 2];
        self.calibrate()?;
        self.device.write(&[RegAddrs::BusVoltage as u8])?;
        self.device.read(&mut buf)?;
        let val = (i16::from_be_bytes(buf) >> 3) as f64;
        Ok(VoltageUnit::volts(val*0.004)) // 4mV step size: 4mV * (1V / 1000 mV)
    }

    pub fn power(&mut self) -> Result<PowerUnit, LinuxI2CError> {
        let mut buf: [u8; 2] = [0; 2];
        self.calibrate()?;
        self.device.write(&[RegAddrs::Power as u8])?;
        self.device.read(&mut buf)?;
        let val = (i16::from_be_bytes(buf)) as f64;
        Ok(PowerUnit::watts(val * self.power_lsb.get_val()))
    }

    pub fn load_current(&mut self) -> Result<CurrentUnit, LinuxI2CError> {
        let mut buf: [u8; 2] = [0; 2];
        self.calibrate()?;
        self.device.write(&[RegAddrs::Current as u8])?;
        self.device.read(&mut buf)?;
        let val = ((i16::from_be_bytes(buf)) as f64)  * self.current_lsb.get_val();
        Ok(CurrentUnit::amps(val))
    }
}
