use crate::device::Device;
use std::{
    convert::TryInto,
    io::{self, Write},
};

pub struct ScreenDevice;

impl ScreenDevice {
    fn erase_screen(&self) {
        print!("\x1b[2J");
    }

    fn move_to(&self, x: usize, y: usize) {
        print!("\x1b[{};{}H", y, x);
    }

    fn set_bold(&self) {
        print!("\x1b[1m");
    }

    fn set_regular(&self) {
        print!("\x1b[0m");
    }
}

impl Device for ScreenDevice {
    fn get_u16(&self, _address: usize) -> Result<u16, String> {
        Ok(0)
    }

    fn get_u8(&self, _address: usize) -> Result<u8, String> {
        Ok(0)
    }

    fn set_u16(&mut self, address: usize, value: u16) -> Result<(), String> {
        let command: u8 = ((value & 0xFF00) >> 8)
            .try_into()
            .map_err(|_| "set_u16: Failed to convert u16 to u8")?;

        match command {
            0xFF => self.erase_screen(),
            0x01 => self.set_bold(),
            0x02 => self.set_regular(),
            _ => {
                // Do nothing
            }
        }
        if command == 0xFF {
            self.erase_screen();
        }

        let character: u8 = (value & 0x00FF)
            .try_into()
            .map_err(|_| "set_u16: Failed to convert u16 to u8")?;
        self.set_u8(address, character)
    }

    fn set_u8(&mut self, address: usize, value: u8) -> Result<(), String> {
        let x = (address % 16) + 1;
        let y = (address / 16) + 1;

        self.move_to(x * 2, y);
        let character = String::from_utf8(vec![value])
            .map_err(|err| format!("set_u16: Failed to get UTF-8 character from u8: {}", err))?;
        print!("{}", character);
        io::stdout().flush().unwrap();

        Ok(())
    }
}
