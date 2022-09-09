use std::vec;

use crate::mem;

#[allow(non_snake_case)]
pub struct CPU {
    pub memory: mem::Mem,
    // 8 Bit HIGH-LOW Registries
    pub AH: u8, // Accumulator High
    pub AL: u8, // Accumulator Low
    pub BH: u8, // Base High
    pub BL: u8, // Base Low
    pub CH: u8, // Counter High
    pub CL: u8, // Counter Low
    pub DH: u8, // Data High
    pub DL: u8, // Data Low

    // 16 Bit Combined Registries
    pub AX: u16, // Accumulator Register
    pub BX: u16, // Base Register
    pub CX: u16, // Counter Register
    pub DX: u16, // Data Register

    // Address Registers
    pub SP: u16, // Stack Pointer
    pub BP: u16, // Base Pointer
    pub DI: u16, // Destination Index
    pub SI: u16, // Source Index

    // Flags
    // control Flags
    pub TF: bool, // Trap
    pub DF: bool, // Direction
    pub IF: bool, // Interrupt Enable
    // Status Flags
    pub OF: bool, // Overflow
    pub SF: bool, // Sign
    pub ZF: bool, // Zero
    pub AF: bool, // Auxillary Carry
    pub PF: bool, // Parity
    pub CF: bool, // Carry
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            memory: mem::Mem{ram: vec![0; 256 * 1024]},
            AH: 0,
            AL: 0,
            BH: 0,
            BL: 0,
            CH: 0,
            CL: 0,
            DH: 0,
            DL: 0,

            AX: 0,
            BX: 0,
            CX: 0,
            DX: 0,

            SP: 0,
            BP: 0,
            DI: 0,
            SI: 0,

            TF: false,
            DF: false,
            IF: false,
            OF: false,
            SF: false,
            ZF: false,
            AF: false,
            PF: false,
            CF: false
        }
    }
}

#[allow(non_snake_case)]
impl CPU {
    // function to setup processor
    pub fn initialize(&mut self) {
        self.setRegister("AX", 302)
    }

    // some registers change others and I don't want to handle that manually
    // For Example the AX register is acutally just The AH-AL range treated as one value
    // Therefore these values change when one of them changes
    pub fn setRegister(&mut self,register: &str, value:u16) {
        match register {
            "AH" => {
                self.AH = (value >> 8) as u8;
                self.AX = u16::from((value >> 8) as u8 * 16 + self.AL);
            }

            "AL" => {
                self.AL = (value >> 8) as u8;
                self.AX = u16::from(self.AH * 16 + (value >> 8) as u8);
            }

            "BH" => {
                self.BH = (value >> 8) as u8;
                self.BX = u16::from((value >> 8) as u8 * 16 + self.BL);
            }

            "BL" => {
                self.BL = (value >> 8) as u8;
                self.BX = u16::from(self.BH * 16 + (value >> 8) as u8);
            }

            "CH" => {
                self.CH = (value >> 8) as u8;
                self.CX = u16::from((value >> 8) as u8 * 16 + self.CL);
            }

            "CL" => {
                self.CL = (value >> 8) as u8;
                self.CX = u16::from(self.CH * 16 + (value >> 8) as u8);
            }

            "DH" => {
                self.DH = (value >> 8) as u8;
                self.DX = u16::from((value >> 8) as u8 * 16 + self.DL);
            }

            "DL" => {
                self.DL = (value >> 8) as u8;
                self.DX = u16::from(self.DH * 16 + (value >> 8) as u8);
            }

            "AX" => {
                self.AX = value;
                self.AL = value as u8;
                self.AH = (value >> 8) as u8
            }

            "BX" => {
                self.BX = value;
                self.BL = value as u8;
                self.BH = (value >> 8) as u8
            }

            "CX" => {
                self.CX = value;
                self.CL = value as u8;
                self.CH = (value >> 8) as u8
            }

            "DX" => {
                self.DX = value;
                self.DL = value as u8;
                self.DH = (value >> 8) as u8
            }

            "SP" => {
                self.SP = value;
            }

            "BP" => {
                self.BP = value;
            }

            "DI" => {
                self.DI = value;
            }

            "SI" => {
                self.SI = value;
            }
            _ => {
                return;
            }
        }
    }
}