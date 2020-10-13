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
    const IP: u8 = 0;
    const ACC: u8 = 1;
    const R1: u8 = 2;
    const R2: u8 = 3;

    let mut memory = memory::create_memory(256 * 256);

    memory[0] = instructions::MOV_LIT_REG;
    memory[1] = 0x12; // 0x1234
    memory[2] = 0x34;
    memory[3] = R1;

    memory[4] = instructions::MOV_LIT_REG;
    memory[5] = 0xAB; // 0xABCD
    memory[6] = 0xCD;
    memory[7] = R2;

    memory[8] = instructions::ADD_REG_REG;
    memory[9] = R1;
    memory[10] = R2;

    memory[11] = instructions::MOV_REG_MEM;
    memory[12] = ACC;
    memory[13] = 0x01; // 0x0100
    memory[14] = 0x00;

    let mut cpu = CPU::new(memory);

    println!("{:#?}", cpu);
    cpu.view_memory_at(cpu.get_register("ip")? as usize);
    cpu.view_memory_at(0x0100);

    cpu.step()?;
    println!("{:#?}", cpu);
    cpu.view_memory_at(cpu.get_register("ip")? as usize);
    cpu.view_memory_at(0x0100);
    
    cpu.step()?;
    println!("{:#?}", cpu);
    cpu.view_memory_at(cpu.get_register("ip")? as usize);
    cpu.view_memory_at(0x0100);
    
    cpu.step()?;
    println!("{:#?}", cpu);
    cpu.view_memory_at(cpu.get_register("ip")? as usize);
    cpu.view_memory_at(0x0100);
    
    cpu.step()?;
    println!("{:#?}", cpu);
    cpu.view_memory_at(cpu.get_register("ip")? as usize);
    cpu.view_memory_at(0x0100);

    Ok(())
}
