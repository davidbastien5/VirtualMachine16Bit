use crate::device::Device;

struct Region {
    device: Box<dyn Device>,
    start: u16,
    end: u16,
    remap: bool,
}

pub struct MemoryMapper {
    regions: Vec<Region>,
}

impl MemoryMapper {
    pub fn new() -> MemoryMapper {
        MemoryMapper {
            regions: Vec::new(),
        }
    }

    /// Adds the given mapping to the list of regions.
    pub fn map(&mut self, device: Box<dyn Device>, start: u16, end: u16, remap: bool) {
        let region = Region {
            device,
            start,
            end,
            remap,
        };

        self.regions.insert(0, region);
    }

    /// Finds the corresponding region for the given address.
    fn find_region(&mut self, address: u16) -> Result<&mut Region, String> {
        let region = self
            .regions
            .iter_mut()
            .find(|region| address >= region.start && address <= region.end);

        region.ok_or_else(|| format!("No memory region found for address {}", address))
    }

    /// Returns the u16 value at the given address.
    pub fn get_u16(&mut self, address: u16) -> Result<u16, String> {
        let region = self.find_region(address)?;
        let address = if region.remap {
            address - region.start
        } else {
            address
        };

        region.device.get_u16(address as usize)
    }

    /// Returns the u8 value at the given address.
    pub fn get_u8(&mut self, address: u16) -> Result<u8, String> {
        let region = self.find_region(address)?;
        let address = if region.remap {
            address - region.start
        } else {
            address
        };

        region.device.get_u8(address as usize)
    }

    /// Sets the given u16 value at the given address.
    pub fn set_u16(&mut self, address: u16, value: u16) -> Result<(), String> {
        let region = self.find_region(address)?;
        let address = if region.remap {
            address - region.start
        } else {
            address
        };

        region.device.set_u16(address as usize, value)
    }

    /// Sets the given u16 value at the given address.
    pub fn set_u8(&mut self, address: u16, value: u8) -> Result<(), String> {
        let region = self.find_region(address)?;
        let address = if region.remap {
            address - region.start
        } else {
            address
        };

        region.device.set_u8(address as usize, value)
    }
}