use std::{convert::TryFrom, io, process};
use virtual_machine16_bit::{cpu::CPU, device::Device, instructions, memory::Memory};

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

    let mut memory = Memory::new(256 * 256);

    let subroutine_address: usize = 0x3000;

    memory.set_u8(0, instructions::PSH_LIT)?;
    memory.set_u16(1, 0x3333)?;

    memory.set_u8(3, instructions::PSH_LIT)?;
    memory.set_u16(4, 0x2222)?;

    memory.set_u8(6, instructions::PSH_LIT)?;
    memory.set_u16(7, 0x1111)?;

    memory.set_u8(9, instructions::MOV_LIT_REG)?;
    memory.set_u16(10, 0x1234)?;
    memory.set_u8(12, R1)?;

    memory.set_u8(13, instructions::MOV_LIT_REG)?;
    memory.set_u16(14, 0x5678)?;
    memory.set_u8(16, R4)?;

    memory.set_u8(17, instructions::PSH_LIT)?;
    memory.set_u16(18, 0x0000)?;

    memory.set_u8(20, instructions::CAL_LIT)?;
    memory.set_u16(
        21,
        u16::try_from(subroutine_address).map_err(|_| "run: Failed to convert usize to u8")?,
    )?;

    memory.set_u8(23, instructions::PSH_LIT)?;
    memory.set_u16(24, 0x4444)?;

    // Subroutine
    memory.set_u8(subroutine_address, instructions::PSH_LIT)?;
    memory.set_u16(subroutine_address + 1, 0x0102)?;

    memory.set_u8(subroutine_address + 3, instructions::PSH_LIT)?;
    memory.set_u16(subroutine_address + 4, 0x0304)?;

    memory.set_u8(subroutine_address + 6, instructions::PSH_LIT)?;
    memory.set_u16(subroutine_address + 7, 0x0506)?;

    memory.set_u8(subroutine_address + 9, instructions::MOV_LIT_REG)?;
    memory.set_u16(subroutine_address + 10, 0x0708)?;
    memory.set_u8(subroutine_address + 12, R1)?;

    memory.set_u8(subroutine_address + 13, instructions::MOV_LIT_REG)?;
    memory.set_u16(subroutine_address + 14, 0x090A)?;
    memory.set_u8(subroutine_address + 16, R8)?;

    memory.set_u8(subroutine_address + 17, instructions::RET)?;

    let mut cpu = CPU::new(memory)?;

    println!("{:#?}", cpu);
    cpu.view_memory_at(cpu.get_register("ip")? as usize, None);
    cpu.view_memory_at(0xFFFF - 1 - 42, Some(44));

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "q" {
            break;
        }

        cpu.step()?;
        println!("{:#?}", cpu);
        cpu.view_memory_at(cpu.get_register("ip")? as usize, None);
        cpu.view_memory_at(0xFFFF - 1 - 42, Some(44));
    }

    Ok(())
}
