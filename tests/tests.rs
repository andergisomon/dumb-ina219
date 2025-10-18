use dumb_ina219::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_units() {
        let mut voltage = VoltageUnit::volts(5.0);
        assert_eq!(voltage.get_val(), 5.0);
        voltage.set_val(3.3);
        assert_eq!(voltage.get_val(), 3.3);

        let current = CurrentUnit::milliamps(500.0);
        assert_eq!(current.get_val(), 500.0);
    }
}
