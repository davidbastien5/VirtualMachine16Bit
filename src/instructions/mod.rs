pub const MOV_LIT_REG: u8     = 0x10;
pub const MOV_REG_REG: u8     = 0x11;
pub const MOV_REG_MEM: u8     = 0x12;
pub const MOV_MEM_REG: u8     = 0x13;
pub const MOV_LIT_MEM: u8     = 0x1B;
pub const MOV_REG_PTR_REG: u8 = 0x1C;
pub const MOV_LIT_OFF_REG: u8 = 0x1D;

pub const ADD_REG_REG: u8     = 0x14;
pub const ADD_LIT_REG: u8     = 0x3F;
pub const SUB_LIT_REG: u8     = 0x16;
pub const SUB_REG_LIT: u8     = 0x1E;
pub const SUB_REG_REG: u8     = 0x1F;
pub const INC_REG: u8         = 0x35;
pub const DEC_REG: u8         = 0x36;
pub const MUL_LIT_REG: u8     = 0x20;
pub const MUL_REG_REG: u8     = 0x21;

pub const JMP_NOT_EQ: u8      = 0x15;
pub const PSH_LIT: u8         = 0x17;
pub const PSH_REG: u8         = 0x18;
pub const POP: u8             = 0x1A;
pub const CAL_LIT: u8         = 0x5E;
pub const CAL_REG: u8         = 0x5F;
pub const RET: u8             = 0x60;
pub const HLT: u8             = 0xFF;