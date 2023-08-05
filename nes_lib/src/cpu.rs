use serde_json::json;
use tracing::info;

#[derive(Debug)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    pub memory: Vec<u8>,
    pub is_running: bool,
    pub debug_mode: bool,
    pub monitored_memory_range: (usize, usize),
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: vec![0; 0xFFFF],
            is_running: false,
            debug_mode: false,
            monitored_memory_range: (0x0000, 15),
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn sta(&mut self, value: u8) {
        self.memory[value as usize] = self.register_a;
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    pub fn run(&mut self) {
        info!("Starting to interpret bytes");
        self.is_running = true;

        loop {
            let opscode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            match opscode {
                0xA9 => {
                    let param = self.mem_read(self.program_counter);
                    self.program_counter += 1;
                    self.lda(param);
                }
                0xAA => self.tax(),
                0x85 => {
                    let param = self.mem_read(self.program_counter);
                    self.program_counter += 1;
                    self.sta(param);
                }
                0x00 => {
                    break;
                }
                _ => todo!(""),
            }
        }

        self.is_running = false;
    }

    pub fn to_cpu_json(&self) -> String {
        let mut monitored_memory: Vec<(String, Vec<u8>)> = Vec::new();
        for i in 0..self.monitored_memory_range.1 {
            let start_index = (self.monitored_memory_range.0 + i * 64) as usize;
            let end_index = (self.monitored_memory_range.0 + (i + 1) * 64) as usize;
            if start_index < self.memory.len() {
                let chunk = if end_index <= self.memory.len() {
                    self.memory[start_index..end_index].to_vec()
                } else {
                    self.memory[start_index..].to_vec()
                };
                let address = format!("{:02X}", start_index);
                monitored_memory.push(("0x".to_owned() + &address, chunk));
            }
        }

        let cpu_state = json!({
            "register_a": self.register_a,
            "register_x": self.register_x,
            "register_y": self.register_y,
            "status": self.status,
            "program_counter": self.program_counter,
            "memory": monitored_memory,
            "is_running": self.is_running,
            "debug_mode": self.debug_mode
        });

        cpu_state.to_string()
    }

    pub fn set_memory_start_address(&mut self, start: usize) {
        self.monitored_memory_range = (start, 10);
    }
}

#[cfg(test)]
mod test {
    use super::CPU;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status & 0b0000_0010 == 0b10);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x80, 0x00]);
        assert_eq!(cpu.register_a, 0x80);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);
    }
}
