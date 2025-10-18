use core::marker::PhantomData; // no need to bring std into this
// need phantomdata as a zero cost dummy to tell the compiler we're doing something with the generic type
// we're just trying to enforce SI units into the type system. why? because i have too much free time and why the hell not

pub trait Settable {
    fn set_val(&mut self, new_val: f64);
}

pub trait Gettable {
    fn get_val(&self) -> f64;
}

#[derive(Debug, Clone, Copy)]
pub enum UnitKind {
    Base,
    Milli,
}

#[derive(Debug, Clone, Copy)]
pub struct Unit<T> {
    val: f64,
    kind: UnitKind,
    _marker: PhantomData<T>,
}

impl<T> Unit<T> {
    pub fn new(val: f64, kind: UnitKind) -> Self {
        Self {
            val,
            kind,
            _marker: PhantomData,
        }
    }

    pub fn base(val: f64) -> Self {
        Self::new(val, UnitKind::Base)
    }

    pub fn milli(val: f64) -> Self {
        Self::new(val, UnitKind::Milli)
    }
}

impl<T> Gettable for Unit<T> {
    fn get_val(&self) -> f64 {
        self.val
    }
}

impl<T> Settable for Unit<T> {
    fn set_val(&mut self, new_val: f64) {
        self.val = new_val;
    }
}

pub struct Voltage;
pub struct Current;
pub struct Resistance;
pub struct Power;

pub type VoltageUnit = Unit<Voltage>;
pub type CurrentUnit = Unit<Current>;
pub type ResistanceUnit = Unit<Resistance>;
pub type PowerUnit = Unit<Power>;

impl VoltageUnit {
    pub fn volts(val: f64) -> Self { Self::base(val) }
    pub fn millivolts(val: f64) -> Self { Self::milli(val) }
}

impl CurrentUnit {
    pub fn amps(val: f64) -> Self { Self::base(val) }
    pub fn milliamps(val: f64) -> Self { Self::milli(val) }
}

impl ResistanceUnit {
    pub fn ohms(val: f64) -> Self { Self::base(val) }
    pub fn milliohms(val: f64) -> Self { Self::milli(val) }
}

impl PowerUnit {
    pub fn watts(val: f64) -> Self { Self::base(val) }
    pub fn milliwatts(val: f64) -> Self { Self::milli(val) }
}
