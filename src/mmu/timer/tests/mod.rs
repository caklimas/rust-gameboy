use crate::addresses::timer::*;
use super::Timer;

/*
DIVIDER_REGISTER => self.divider,
TIMER_COUNTER => self.counter,
TIMER_MODULO => self.modulo,
TIMER_CONTROL => self.control.get(),
*/

#[test]
fn read_divider_test() {
    let value = 1;
    let mut timer: Timer = Default::default();
    timer.divider = value;

    let result = timer.read(DIVIDER_REGISTER);

    assert_eq!(value, result);
}

#[test]
fn read_counter_test() {
    let value = 1;
    let mut timer: Timer = Default::default();
    timer.counter = value;

    let result = timer.read(TIMER_COUNTER);

    assert_eq!(value, result);
}

#[test]
fn read_modulo_test() {
    let value = 1;
    let mut timer: Timer = Default::default();
    timer.modulo = value;

    let result = timer.read(TIMER_MODULO);

    assert_eq!(value, result);
}

#[test]
fn read_control_test() {
    let value = 1;
    let mut timer: Timer = Default::default();
    timer.control.set(value);

    let result = timer.read(TIMER_CONTROL);

    assert_eq!(value, result);
}

#[test]
fn write_divider_test() {
    let value = 1;
    let mut timer: Timer = Default::default();
    timer.divider = value;

    timer.write(DIVIDER_REGISTER, value);

    assert_eq!(0, timer.divider);
}

#[test]
fn write_counter_test() {
    let value = 1;
    let mut timer: Timer = Default::default();

    timer.write(TIMER_COUNTER, value);

    assert_eq!(value, timer.counter);
}

#[test]
fn write_modulo_test() {
    let value = 1;
    let mut timer: Timer = Default::default();

    timer.write(TIMER_MODULO, value);

    assert_eq!(value, timer.modulo);
}

#[test]
fn write_control_test() {
    let value = 1;
    let mut timer: Timer = Default::default();

    timer.write(TIMER_CONTROL, value);

    assert_eq!(value, timer.control.get());
}