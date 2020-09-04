use super::ClockCycle;

impl super::super::Cpu {
    pub fn ei(&mut self) -> ClockCycle {
        self.interrupt_master_enable = true;
        (1, 4)
    }

    pub fn di(&mut self) -> ClockCycle {
        self.interrupt_master_enable = false;
        (1, 4)
    }

    pub fn halt(&mut self) -> ClockCycle {
        self.halted = true;
        (1, 4)
    }

    pub fn nop(&self) -> ClockCycle {
        (1, 4)
    }

    pub fn prefix_cb(&mut self) -> ClockCycle {
        self.cb_opcode = true;
        (1, 4)
    }

    pub fn stop(&mut self) -> ClockCycle {
        println!("Stop");
        self.stopped = true;
        (2, 4)
    }
}