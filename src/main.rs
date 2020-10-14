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
    const R3: u8 = 4;
    const R4: u8 = 5;
    const R5: u8 = 6;
    const R6: u8 = 7;
    const R7: u8 = 8;
    const R8: u8 = 9;
    const SP: u8 = 10;
    const FP: u8 = 11;

    let mut memory = memory::create_memory(256 * 256);

    memory[0] = instructions::MOV_LIT_REG;
    memory[1] = 0x51; // 0x5151
    memory[2] = 0x51;
    memory[3] = R1;

    memory[4] = instructions::MOV_LIT_REG;
    memory[5] = 0x42; // 0x4242
    memory[6] = 0x42;
    memory[7] = R2;

    memory[8] = instructions::PSH_REG;
    memory[9] = R1;

    memory[10] = instructions::PSH_REG;
    memory[11] = R2;

    memory[12] = instructions::POP;
    memory[13] = R1;

    memory[14] = instructions::POP;
    memory[15] = R2;

    let mut cpu = CPU::new(memory)?;

    println!("{:#?}", cpu);
    cpu.view_memory_at(cpu.get_register("ip")? as usize);
    cpu.view_memory_at(0xFFF8);

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
        cpu.view_memory_at(0xFFF8);
    }

    Ok(())
}
