use crate::memory;
use std::collections::HashMap;

const REGISTER_NAMES: [&str; 10] = ["ip", "acc", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8"];

#[derive(Debug)]
pub struct CPU {
    memory: Box<[u8]>,
    register_map: HashMap<String, usize>,
    registers: Box<[u8]>,
}

impl CPU {
    pub fn new(memory: Box<[u8]>) -> Result<CPU, String> {
        let register_map: HashMap<String, usize> = REGISTER_NAMES
            .iter()
            .enumerate()
            .map(|(index, register_name)| (String::from(*register_name), index * 2))
            .collect();

        let registers = memory::create_memory(REGISTER_NAMES.len() * 2);

        Ok(CPU {
            memory,
            register_map,
            registers,
        })
    }

    pub fn get_register(&self, name: &str) -> Result<u16, String> {
        let index = self
            .register_map
            .get(name)
            .ok_or(format!("get_register: No such register '{}'", name))?;

        let bytes: [u8; 2] = [self.registers[*index], self.registers[*index + 1]];
        Ok(u16::from_be_bytes(bytes))
    }

    pub fn set_register(&mut self, name: &str, value: u16) -> Result<(), String> {
        let index = self
            .register_map
            .get(name)
            .ok_or(format!("set_register: No such register '{}'", name))?;

        let bytes = value.to_be_bytes();
        self.registers[*index] = bytes[0];
        self.registers[*index + 1] = bytes[1];
        Ok(())
    }
}
