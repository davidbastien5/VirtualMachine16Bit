pub trait Device {
    fn get_u16(&self, address: usize) -> Result<u16, String>;
    fn get_u8(&self, address: usize) -> Result<u8, String>;
    fn set_u16(&mut self, address: usize, value: u16) -> Result<(), String>;
    fn set_u8(&mut self, address: usize, value: u8) -> Result<(), String>;
}