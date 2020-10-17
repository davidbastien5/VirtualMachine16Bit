use std::{convert::TryFrom, process};
use virtual_machine16_bit::{
    cpu::CPU, device::Device, instructions, memory::Memory, memory_mapper::MemoryMapper,
    screen_device::ScreenDevice,
};

fn main() {
    process::exit(match run() {
        Ok(_) => 0,
        Err(error) => {
            eprintln!("error: {}", error);
            1
        }
    });
}

fn run() -> Result<(), String> {
    const IP: u8 = 0;
    const ACC: u8 = 1;
    const R1: u8 = 2;
    const R2: u8 = 3;
    const R3: u8 = 4;
    const R4: u8 = 5;
    const R5: u8 = 6;
    const R6: u8 = 7;
    const R7: u8 = 8;
    const R8: u8 = 9;
    const SP: u8 = 10;
    const FP: u8 = 11;

    let mut memory_mapper = MemoryMapper::new();

    let mut memory = Memory::new(256 * 256);

    let mut index = 0;
    let mut write_char_to_screen = |character: char, command: u8, position: u16| {
        memory.set_u8(index, instructions::MOV_LIT_REG).unwrap();
        index = index + 1;
        memory.set_u8(index, command).unwrap();
        index = index + 1;
        memory.set_u8(index, character as u8).unwrap();
        index = index + 1;
        memory.set_u8(index, R1).unwrap();
        index = index + 1;

        memory.set_u8(index, instructions::MOV_REG_MEM).unwrap();
        index = index + 1;
        memory.set_u8(index, R1).unwrap();
        index = index + 1;
        memory.set_u16(index, 0x3000 + position).unwrap();
        index = index + 2;
    };

    write_char_to_screen(' ', 0xFF, 0);

    for i in 0u16..=0xFF {
        let command: u8 = if i % 2 == 0 { 0x01 } else { 0x02 };
        write_char_to_screen('*', command, i);
    }

    memory.set_u8(index, instructions::HLT)?;
    memory_mapper.map(Box::new(memory), 0, 0xFFFF, false);
    memory_mapper.map(Box::new(ScreenDevice), 0x3000, 0x30FF, true);

    let mut cpu = CPU::new(memory_mapper)?;
    cpu.run()?;

    Ok(())
}
