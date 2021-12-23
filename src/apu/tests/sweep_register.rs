use crate::apu::sweep_register::SweepRegister;

#[test]
fn get_sweep_time_test() {
    let epsilon = 0.0001;
    let sweep_register = SweepRegister(0b0111_0000);

    let time = sweep_register.get_sweep_time();

    assert_eq!(true, ((0.0547 - time).abs() < epsilon));
}
