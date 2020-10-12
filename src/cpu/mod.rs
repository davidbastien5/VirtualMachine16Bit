use crate::{instructions, memory};
use std::{collections::HashMap, fmt};

const REGISTER_NAMES: [&str; 10] = ["ip", "acc", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8"];

pub struct CPU {
    memory: Box<[u8]>,
    register_map: HashMap<String, usize>,
    registers: Box<[u8]>,
}

impl CPU {
    /// Creates a new CPU instance with the given memory.
    pub fn new(memory: Box<[u8]>) -> CPU {
        let register_map: HashMap<String, usize> = REGISTER_NAMES
            .iter()
            .enumerate()
            .map(|(index, register_name)| (String::from(*register_name), index * 2))
            .collect();

        let registers = memory::create_memory(REGISTER_NAMES.len() * 2);

        CPU {
            memory,
            register_map,
            registers,
        }
    }

    /// Gets the value in the given register.
    pub fn get_register(&self, name: &str) -> Result<u16, String> {
        let index = self
            .register_map
            .get(name)
            .ok_or(format!("get_register: No such register '{}'", name))?;

        let bytes: [u8; 2] = [self.registers[*index], self.registers[*index + 1]];
        Ok(u16::from_be_bytes(bytes))
    }

    /// Sets the given value to the given register.
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

    /// Fetches the next 8-bit instruction and increments the instruction pointer.
    pub fn fetch(&mut self) -> Result<u8, String> {
        let instruction_address = self.get_register("ip")?;
        let instruction = self.memory[instruction_address as usize];
        self.set_register("ip", instruction_address + 1)?;

        Ok(instruction)
    }

    /// Fetches the next 16-bit instruction and increments the instruction pointer.
    pub fn fetch16(&mut self) -> Result<u16, String> {
        let instruction_address = self.get_register("ip")?;
        let bytes: [u8; 2] = [
            self.memory[instruction_address as usize],
            self.memory[(instruction_address + 1) as usize],
        ];
        let instruction = u16::from_be_bytes(bytes);
        self.set_register("ip", instruction_address + 2)?;

        Ok(instruction)
    }

    /// Executes the given instruction.
    pub fn execute(&mut self, instruction: u8) -> Result<(), String> {
        match instruction {
            // Move literal to the r1 register
            instructions::MOV_LIT_R1 => {
                let literal = self.fetch16()?;
                self.set_register("r1", literal)?;
            }

            // Move literal to the r2 register
            instructions::MOV_LIT_R2 => {
                let literal = self.fetch16()?;
                self.set_register("r2", literal)?;
            }

            // Add register to register
            instructions::ADD_REG_REG => {
                let register1 = self.fetch()? as usize;
                let register2 = self.fetch()? as usize;
                let register_value1 = u16::from_be_bytes([
                    self.registers[register1 * 2],
                    self.registers[register1 * 2 + 1],
                ]);
                let register_value2 = u16::from_be_bytes([
                    self.registers[register2 * 2],
                    self.registers[register2 * 2 + 1],
                ]);
                self.set_register("acc", register_value1 + register_value2)?;
            }

            _ => return Err(format!("execute: No such instruction '{}'", instruction)),
        }

        Ok(())
    }

    /// Executes the next instruction.
    pub fn step(&mut self) -> Result<(), String> {
        let instruction = self.fetch()?;
        self.execute(instruction)
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CPU")
            .field("ip", &format!("{:#06X}", self.get_register("ip").unwrap()))
            .field("acc", &format!("{:#06X}", self.get_register("acc").unwrap()))
            .field("r1", &format!("{:#06X}", self.get_register("r1").unwrap()))
            .field("r2", &format!("{:#06X}", self.get_register("r2").unwrap()))
            .field("r3", &format!("{:#06X}", self.get_register("r3").unwrap()))
            .field("r4", &format!("{:#06X}", self.get_register("r4").unwrap()))
            .field("r5", &format!("{:#06X}", self.get_register("r5").unwrap()))
            .field("r6", &format!("{:#06X}", self.get_register("r6").unwrap()))
            .field("r7", &format!("{:#06X}", self.get_register("r7").unwrap()))
            .field("r8", &format!("{:#06X}", self.get_register("r8").unwrap()))
            .finish()
    }
}
