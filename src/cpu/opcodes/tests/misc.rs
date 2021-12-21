use super::super::super::Cpu;

#[test]
fn di_test() {
    let mut cpu: Cpu = Default::default();
    cpu.interrupt_master_enable = true;

    cpu.di();

    assert_eq!(false, cpu.interrupt_master_enable);
}

#[test]
fn ei_test() {
    let mut cpu: Cpu = Default::default();
    cpu.interrupt_master_enable = false;

    cpu.ei();

    assert_eq!(true, cpu.interrupt_master_enable);
}

#[test]
fn halt_test() {
    let mut cpu: Cpu = Default::default();
    cpu.halted = false;

    cpu.halt();

    assert_eq!(true, cpu.halted);
}

#[test]
fn prefix_cb_test() {
    let mut cpu: Cpu = Default::default();
    cpu.cb_opcode = false;

    cpu.prefix_cb();

    assert_eq!(true, cpu.cb_opcode);
}

#[test]
fn stop_test() {
    let mut cpu: Cpu = Default::default();
    cpu.stopped = false;

    cpu.stop();

    assert_eq!(true, cpu.stopped);
}