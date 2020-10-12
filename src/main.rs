use std::process;
use virtual_machine16_bit::{cpu::CPU, memory};

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
    let cpu = CPU::new(memory::create_memory(1))?;
    println!("{:?}", cpu);
    println!("{}", cpu.get_register("ip")?);
    Ok(())
}
