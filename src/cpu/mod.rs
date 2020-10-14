use crate::{instructions, memory};
use std::{collections::HashMap, convert::TryFrom, fmt};

const REGISTER_NAMES: [&str; 12] = [
    "ip", "acc", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "sp", "fp",
];

pub struct CPU {
    memory: Box<[u8]>,
    register_map: HashMap<String, usize>,
    registers: Box<[u8]>,
}

impl CPU {
    /// Creates a new CPU instance with the given memory.
    pub fn new(memory: Box<[u8]>) -> Result<CPU, String> {
        let register_map: HashMap<String, usize> = REGISTER_NAMES
            .iter()
            .enumerate()
            .map(|(index, register_name)| (String::from(*register_name), index * 2))
            .collect();

        let registers = memory::create_memory(REGISTER_NAMES.len() * 2);

        let memory_len = memory.len();

        let mut cpu = CPU {
            memory,
            register_map,
            registers,
        };

        let last_memory_address =
            u16::try_from(memory_len - 1).map_err(|_| "new: Memory length is more than 16-bits")?;
        cpu.set_register("sp", last_memory_address - 1)?;
        cpu.set_register("fp", last_memory_address - 1)?;

        Ok(cpu)
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

    /// Fetches the stored register index.
    pub fn fetch_register_index(&mut self) -> Result<usize, String> {
        Ok((self.fetch()? as usize % REGISTER_NAMES.len()) * 2)
    }

    /// Pushes the given value onto the stack and moves the stack pointer.
    pub fn push(&mut self, value: u16) -> Result<(), String> {
        let value = value.to_be_bytes();
        let address = self.get_register("sp")?;
        self.memory[address as usize] = value[0];
        self.memory[(address + 1) as usize] = value[1];
        self.set_register("sp", address - 2)?;

        Ok(())
    }

    /// Moves the stack pointer and returns the value at that memory location.
    pub fn pop(&mut self) -> Result<u16, String> {
        let next_sp_address = self.get_register("sp")? + 2;
        self.set_register("sp", next_sp_address)?;
        Ok(u16::from_be_bytes([
            self.memory[next_sp_address as usize],
            self.memory[(next_sp_address + 1) as usize],
        ]))
    }

    /// Executes the given instruction.
    pub fn execute(&mut self, instruction: u8) -> Result<(), String> {
        match instruction {
            // Move literal into register
            instructions::MOV_LIT_REG => {
                let literal = self.fetch16()?.to_be_bytes();
                let register = self.fetch_register_index()?;
                self.registers[register] = literal[0];
                self.registers[register + 1] = literal[1];
            }

            // Move register to register
            instructions::MOV_REG_REG => {
                let register_from = self.fetch_register_index()?;
                let register_to = self.fetch_register_index()?;
                let value = [
                    self.registers[register_from],
                    self.registers[register_from + 1],
                ];
                self.registers[register_to] = value[0];
                self.registers[register_to + 1] = value[1];
            }

            // Move register to memory
            instructions::MOV_REG_MEM => {
                let register_from = self.fetch_register_index()?;
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
                let register_to = self.fetch_register_index()?;
                let value = [self.memory[address], self.memory[address + 1]];
                self.registers[register_to] = value[0];
                self.registers[register_to + 1] = value[1];
            }

            // Add register to register
            instructions::ADD_REG_REG => {
                let register1 = self.fetch_register_index()?;
                let register2 = self.fetch_register_index()?;
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

            // Push literal
            instructions::PSH_LIT => {
                let literal = self.fetch16()?;
                self.push(literal)?;
            }

            // Push register
            instructions::PSH_REG => {
                let register = self.fetch_register_index()?;
                let value =
                    u16::from_be_bytes([self.registers[register], self.registers[register + 1]]);
                self.push(value)?;
            }

            // Pop
            instructions::POP => {
                let register = self.fetch_register_index()?;
                let value = self.pop()?.to_be_bytes();
                self.registers[register] = value[0];
                self.registers[register + 1] = value[1];
            }

            _ => {
                // Do nothing
            }
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
            .field("sp", &format!("{:#06X}", self.get_register("sp").unwrap()))
            .field("fp", &format!("{:#06X}", self.get_register("fp").unwrap()))
            .finish()
    }
}
