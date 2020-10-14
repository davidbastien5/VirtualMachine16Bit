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
            // Move literal into register
            instructions::MOV_LIT_REG => {
                let literal = self.fetch16()?.to_be_bytes();
                let register = self.fetch()? as usize % REGISTER_NAMES.len() * 2;
                self.registers[register] = literal[0];
                self.registers[register + 1] = literal[1];
            }

            // Move register to register
            instructions::MOV_REG_REG => {
                let register_from = self.fetch()? as usize % REGISTER_NAMES.len() * 2;
                let register_to = self.fetch()? as usize % REGISTER_NAMES.len() * 2;
                let value = [
                    self.registers[register_from],
                    self.registers[register_from + 1],
                ];
                self.registers[register_to] = value[0];
                self.registers[register_to + 1] = value[1];
            }

            // Move register to memory
            instructions::MOV_REG_MEM => {
                let register_from = self.fetch()? as usize % REGISTER_NAMES.len() * 2;
                let address = self.fetch16()? as usize;
                let value = [
                    self.registers[register_from],
                    self.registers[register_from + 1],
                ];
                self.memory[address] = value[0];
                self.memory[address + 1] = value[1];
            }

            // Move memory to register
            instructions::MOV_MEM_REG => {
                let address = self.fetch16()? as usize;
                let register_to = self.fetch()? as usize % REGISTER_NAMES.len() * 2;
                let value = [self.memory[address], self.memory[address + 1]];
                self.registers[register_to] = value[0];
                self.registers[register_to + 1] = value[1];
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

            // Jump if not equal
            instructions::JMP_NOT_EQ => {
                let literal = self.fetch16()?;
                let address = self.fetch16()?;

                if literal != self.get_register("acc")? {
                    self.set_register("ip", address)?;
                }
            }

            _ => {
                // Do nothing
            },
        }

        Ok(())
    }

    /// Executes the next instruction.
    pub fn step(&mut self) -> Result<(), String> {
        let instruction = self.fetch()?;
        self.execute(instruction)
    }

    /// Prints the memory at the given address.
    pub fn view_memory_at(&self, address: usize) {
        let values = format!(
            "{:#04X} {:#04X} {:#04X} {:#04X} {:#04X} {:#04X} {:#04X} {:#04X}",
            self.memory[address],
            self.memory[address + 1],
            self.memory[address + 2],
            self.memory[address + 3],
            self.memory[address + 4],
            self.memory[address + 5],
            self.memory[address + 6],
            self.memory[address + 7],
        );
        println!("{:#06X}: {}", address, values);
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CPU")
            .field("ip", &format!("{:#06X}", self.get_register("ip").unwrap()))
            .field(
                "acc",
                &format!("{:#06X}", self.get_register("acc").unwrap()),
            )
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
