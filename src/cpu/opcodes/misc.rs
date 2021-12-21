impl super::super::Cpu {
    pub fn ei(&mut self) -> u16 {
        self.interrupt_master_enable = true;
        4
    }

    pub fn di(&mut self) -> u16 {
        self.interrupt_master_enable = false;
        4
    }

    pub fn halt(&mut self) -> u16 {
        self.halted = true;
        4
    }

    pub fn nop(&self) -> u16 {
        4
    }

    pub fn prefix_cb(&mut self) -> u16 {
        self.cb_opcode = true;
        4
    }

    pub fn stop(&mut self) -> u16 {
        println!("Stop");
        self.stopped = true;
        4
    }
}
