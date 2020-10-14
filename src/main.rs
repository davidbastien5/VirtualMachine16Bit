use std::{io, process};
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

    let mut memory = memory::create_memory(256 * 256);

    memory[0] = instructions::MOV_MEM_REG;
    memory[1] = 0x01; // 0x0100
    memory[2] = 0x00;
    memory[3] = R1;

    memory[4] = instructions::MOV_LIT_REG;
    memory[5] = 0x00; // 0x0001
    memory[6] = 0x01;
    memory[7] = R2;

    memory[8] = instructions::ADD_REG_REG;
    memory[9] = R1;
    memory[10] = R2;

    memory[11] = instructions::MOV_REG_MEM;
    memory[12] = ACC;
    memory[13] = 0x01; // 0x0100
    memory[14] = 0x00;

    memory[15] = instructions::JMP_NOT_EQ;
    memory[16] = 0x00; // 0x0003
    memory[17] = 0x03;
    memory[18] = 0x00; // 0x0000
    memory[19] = 0x00;

    let mut cpu = CPU::new(memory);

    println!("{:#?}", cpu);
    cpu.view_memory_at(cpu.get_register("ip")? as usize);
    cpu.view_memory_at(0x0100);

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
        cpu.view_memory_at(cpu.get_register("ip")? as usize);
        cpu.view_memory_at(0x0100);
    }

    Ok(())
}
