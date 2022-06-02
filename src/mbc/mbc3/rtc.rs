use std::time;

#[derive(Default)]
pub struct Rtc {
    seconds: u8,
    minutes: u8,
    hours: u8,
    day_counter_lo: u8,
    day_counter_hi: u8,
    t0: u64,
}

impl Rtc {
    pub fn calculate_clock(&mut self) {
        if (self.day_counter_lo & 0x40) == 0 {
            return;
        }
        let t0 = time::UNIX_EPOCH + time::Duration::from_secs(self.t0);
        let difftime = match time::SystemTime::now().duration_since(t0) {
            Ok(n) => n.as_secs(),
            _ => 0,
        };
        self.seconds = (difftime % 60) as u8;
        self.minutes = ((difftime / 60) % 60) as u8;
        self.hours = ((difftime / 3600) % 24) as u8;
        self.day_counter_hi = (difftime / (3600 * 24)) as u8;
    }

    pub fn read(&self, register: u8) -> u8 {
        match register {
            0x08 => self.seconds,
            0x09 => self.minutes,
            0x0A => self.hours,
            0x0B => self.day_counter_hi,
            0x0C => self.day_counter_lo,
            _ => 0x00,
        }
    }

    pub fn write(&mut self, register: u8, data: u8) {
        match register {
            0x08 => self.seconds = data,
            0x09 => self.minutes = data,
            0x0A => self.hours = data,
            0x0B => self.day_counter_hi = data,
            0x0C => self.day_counter_lo = data,
            _ => (),
        }
    }
}
