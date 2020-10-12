use std::process;
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
    let mut memory = memory::create_memory(256);

    memory[0] = instructions::MOV_LIT_R1;
    memory[1] = 0x12; // 0x1234
    memory[2] = 0x34;

    memory[3] = instructions::MOV_LIT_R2;
    memory[4] = 0xAB; // 0xABCD
    memory[5] = 0xCD;

    memory[6] = instructions::ADD_REG_REG;
    memory[7] = 2; // r1 index
    memory[8] = 3; // r2 index

    let mut cpu = CPU::new(memory);
    println!("{:#?}", cpu);
    cpu.step()?;
    println!("{:#?}", cpu);
    cpu.step()?;
    println!("{:#?}", cpu);
    cpu.step()?;
    println!("{:#?}", cpu);
    Ok(())
}
