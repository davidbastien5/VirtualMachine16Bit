use std::{convert::TryFrom, io, process};
use virtual_machine16_bit::{cpu::CPU, instructions, memory};

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

    let mut memory = memory::create_memory(256 * 256);

    let subroutine_address: usize = 0x3000;

    memory[0] = instructions::PSH_LIT;
    memory[1] = 0x33;
    memory[2] = 0x33;

    memory[3] = instructions::PSH_LIT;
    memory[4] = 0x22;
    memory[5] = 0x22;

    memory[6] = instructions::PSH_LIT;
    memory[7] = 0x11;
    memory[8] = 0x11;

    memory[9] = instructions::MOV_LIT_REG;
    memory[10] = 0x12;
    memory[11] = 0x34;
    memory[12] = R1;

    memory[13] = instructions::MOV_LIT_REG;
    memory[14] = 0x56;
    memory[15] = 0x78;
    memory[16] = R4;

    memory[17] = instructions::PSH_LIT;
    memory[18] = 0x00;
    memory[19] = 0x00;

    memory[20] = instructions::CAL_LIT;
    memory[21] = u8::try_from((subroutine_address & 0xff00) >> 8)
        .map_err(|_| "run: Failed to convert usize to u8")?;
    memory[22] = u8::try_from(subroutine_address & 0x00ff)
        .map_err(|_| "run: Failed to convert usize to u8")?;

    memory[23] = instructions::PSH_LIT;
    memory[24] = 0x44;
    memory[25] = 0x44;

    // Subroutine
    memory[subroutine_address] = instructions::PSH_LIT;
    memory[subroutine_address + 1] = 0x01;
    memory[subroutine_address + 2] = 0x02;

    memory[subroutine_address + 3] = instructions::PSH_LIT;
    memory[subroutine_address + 4] = 0x03;
    memory[subroutine_address + 5] = 0x04;

    memory[subroutine_address + 6] = instructions::PSH_LIT;
    memory[subroutine_address + 7] = 0x05;
    memory[subroutine_address + 8] = 0x06;

    memory[subroutine_address + 9] = instructions::MOV_LIT_REG;
    memory[subroutine_address + 10] = 0x07;
    memory[subroutine_address + 11] = 0x08;
    memory[subroutine_address + 12] = R1;

    memory[subroutine_address + 13] = instructions::MOV_LIT_REG;
    memory[subroutine_address + 14] = 0x09;
    memory[subroutine_address + 15] = 0x0A;
    memory[subroutine_address + 16] = R8;

    memory[subroutine_address + 17] = instructions::RET;

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
