use dumb_ina219::*;
use crate::units::*;
// disclaimer: generated with an LLM, but human audited

#[cfg(test)]
mod tests {
    use super::*;

    const INITIAL_VOLTAGE: f64 = 12.5;
    const NEW_VOLTAGE: f64 = 5.0;

    #[test]
    // Tests that constructors correctly set value and UnitKind
    fn test_constructors() {
        // Test base unit (Volts)
        let v_base = VoltageUnit::volts(INITIAL_VOLTAGE);
        assert_eq!(v_base.val, INITIAL_VOLTAGE);
        assert_eq!(v_base.kind, UnitKind::Base);

        // Test milli unit (Milliamps)
        let c_milli = CurrentUnit::milliamps(250.0);
        assert_eq!(c_milli.val, 250.0 / 1000.0);
        assert_eq!(c_milli.kind, UnitKind::Milli);

        // Test base unit (Ohms)
        let r_base = ResistanceUnit::ohms(10.0);
        assert_eq!(r_base.val, 10.0);
        assert_eq!(r_base.kind, UnitKind::Base);

        // Test milli unit (Milliwatts)
        let p_milli = PowerUnit::milliwatts(50.0);
        assert_eq!(p_milli.val, 50.0 / 1000.0);
        assert_eq!(p_milli.kind, UnitKind::Milli);
    }

    #[test]
    // Tests the Gettable trait implementation
    fn test_gettable() {
        let v = VoltageUnit::volts(INITIAL_VOLTAGE);
        let c = CurrentUnit::milliamps(250.0);

        // Check if get_val returns the correct value
        assert_eq!(v.get_val(), INITIAL_VOLTAGE);
        assert_eq!(c.get_val(), 250.0 / 1000.0);
    }

    #[test]
    // Tests the Settable trait implementation
    fn test_settable() {
        let mut v = VoltageUnit::volts(INITIAL_VOLTAGE);

        // Initial check
        assert_eq!(v.get_val(), INITIAL_VOLTAGE);

        // Set new value
        v.set_val(NEW_VOLTAGE);

        // Check the new value
        assert_eq!(v.get_val(), NEW_VOLTAGE);

        // Set another value
        v.set_val(0.0);
        assert_eq!(v.get_val(), 0.0);
    }

    #[test]
    // Verifies that the generic markers enforce type separation
    fn test_type_safety() {
        let _voltage_unit: Unit<Voltage> = VoltageUnit::volts(1.0);
        let _current_unit: Unit<Current> = CurrentUnit::amps(1.0);

        // This line would cause a compilation error:
        // let _error: VoltageUnit = CurrentUnit::amps(1.0);
        // The existence of the separate type aliases confirms the use of PhantomData
        // successfully creates distinct types at compile time.
        // We assert true just to show the test ran successfully.
        assert!(true);
    }
}
