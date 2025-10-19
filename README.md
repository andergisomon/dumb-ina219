# dumb-ina219
A dumb INA219 driver for the Raspberry Pi, hastily made for a course project. Seems to work, YMMV. Tested on a Raspberry Pi 5.

## What works
Everything seems to work from  limited testing.

# Example

```rust
use dumb_ina219::{units::{CurrentUnit, Gettable, ResistanceUnit}, *};
use i2cdev::linux::LinuxI2CError;

const TARGET_ADDR: u8 = 0x44;

fn main() -> Result<(), LinuxI2CError> {
    let mut dev = Ina219::new(
        ResistanceUnit::milliohms(100.0),
        CurrentUnit::milliamps(1000.0),
        TARGET_ADDR)?;
    dev.init()?;
    let current_reading = dev.load_current()?;
    let shunt_voltage_reading = dev.shunt_voltage()?;
    let bus_voltage_reading = dev.bus_voltage()?;
    let power_reading = dev.power()?;


    println!("Load current: {:?} mA", current_reading.get_val()*1000.0);
    println!("Shunt voltage: {:?} mV", shunt_voltage_reading.get_val()*1000.0);
    println!("Bus voltage: {:?} V", bus_voltage_reading.get_val());
    println!("Power: {:?} mW", power_reading.get_val()*1000.0);
    Ok(())
}
```

## Notes to dev
Specify cross comp target to check:
```bash
cross check --target aarch64-unknown-linux-gnu
```

Send to Pi:
```bash
scp -i ~/gipop_plc /Users/ander/Documents/proj/dumb-ina219/target/aarch64-unknown-linux-gnu/release/examples/ina219_xmpl pi@172.30.40.32:/home/pi/palanuk/anc/
```
