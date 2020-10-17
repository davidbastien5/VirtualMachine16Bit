use crate::device::Device;

pub struct Memory {
    memory: Box<[u8]>
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            memory: vec![0; size].into_boxed_slice()
        }
    }
}

impl Device for Memory {
    fn get_u16(&self, address: usize) -> Result<u16, String> {
        Ok(u16::from_be_bytes([
            self.memory[address],
            self.memory[address + 1]
        ]))
    }

    fn get_u8(&self, address: usize) -> Result<u8, String> {
        Ok(self.memory[address])
    }

    fn set_u16(&mut self, address: usize, value: u16) -> Result<(), String> {
        let value = value.to_be_bytes();
        self.memory[address] = value[0];
        self.memory[address + 1] = value[1];
        Ok(())
    }

    fn set_u8(&mut self, address: usize, value: u8) -> Result<(), String> {
        self.memory[address] = value;
        Ok(())
    }
}