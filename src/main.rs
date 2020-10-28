use std::process;
/*use virtual_machine16_bit::virtual_machine::{
    cpu::CPU, device::Device, instructions, memory::Memory, memory_mapper::MemoryMapper,
    screen_device::ScreenDevice,
};*/
use virtual_machine16_bit::assembler::parser;

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
    
    let test = "hlt";
    
    let ast = parser::instructions::instruction(test)
        .map(|parser_result| parser_result.1)
        .map_err(|err| format!("Unable to parse the instruction: {}", err))?;

    println!("{}", test);
    println!("{:#?}", ast);

    Ok(())
}
