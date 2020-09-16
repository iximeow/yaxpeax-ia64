use yaxpeax_arch::{Arch, AddressDiff, Decoder, LengthedInstruction, NoColors, YaxColors};

use core::fmt;

/// TODO: ia64 reference doc

pub struct IA64;

impl Arch for IA64 {
    type Address = u64;
    type Instruction = Instruction;
    type DecodeError = DecodeError;
    type Decoder = InstDecoder;
    type Operand = Operand;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {}
impl yaxpeax_arch::LengthedInstruction for Instruction {
    type Unit = yaxpeax_arch::AddressDiff<u64>;
    fn len(&self) -> Self::Unit { AddressDiff::from_const(1) }
    fn min_size() -> Self::Unit { AddressDiff::from_const(1) }
}
impl yaxpeax_arch::Instruction for Instruction {
    fn well_defined(&self) -> bool {
        true
    }
}
impl Default for Instruction {
    fn default() -> Self {
        Instruction { }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    ExhaustedInput,
    BadOpcode,
    BadOperand,
    BadBundle,
}
impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeError::ExhaustedInput => f.write_str("exhausted input"),
            DecodeError::BadBundle => f.write_str("bad bundle"),
            DecodeError::BadOpcode => f.write_str("bad opcode"),
            DecodeError::BadOperand => f.write_str("bad operand"),
        }
    }
}
impl yaxpeax_arch::DecodeError for DecodeError {
    fn data_exhausted(&self) -> bool {
        if let DecodeError::ExhaustedInput = self {
            true
        } else {
            false
        }
    }
    fn bad_opcode(&self) -> bool {
        if let DecodeError::BadBundle = self {
            true
        } else if let DecodeError::BadOpcode = self {
            true
        } else {
            false
        }
    }
    fn bad_operand(&self) -> bool {
        if let DecodeError::BadOperand = self {
            true
        } else {
            false
        }
    }
}
#[derive(Default)]
pub struct InstDecoder {}
#[derive(Debug)]
pub enum Operand {}

impl Decoder<Instruction> for InstDecoder {
    type Error = DecodeError;

    fn decode_into<T: IntoIterator<Item=u8>>(&self, inst: &mut Instruction, bytes: T) -> Result<(), Self::Error> {
        let mut bytes_iter = bytes.into_iter();
        let bundle = bytes_iter.next().ok_or(DecodeError::ExhaustedInput)?;
        let bundle_tag = bundle & 0x1f;
        let bundle_desc = match bundle_tag {
            0x00 => { "M I I " },
            0x01 => { "M I I|" },
            0x02 => { "M I|I " },
            0x03 => { "M I|I|" },
            0x04 => { "M L X " },
            0x05 => { "M L X|" },
            0x06 => { return Err(DecodeError::BadBundle) },
            0x07 => { return Err(DecodeError::BadBundle) },
            0x08 => { "M M I " },
            0x09 => { "M M I|" },
            0x0a => { "M|M I " },
            0x0b => { "M|M I|" },
            0x0c => { "M F I " },
            0x0d => { "M F I|" },
            0x0e => { "M M F " },
            0x0f => { "M M F|" },
            0x10 => { "M I B " },
            0x11 => { "M I B|" },
            0x12 => { "M B B " },
            0x13 => { "M B B|" },
            0x14 => { return Err(DecodeError::BadBundle) },
            0x15 => { return Err(DecodeError::BadBundle) },
            0x16 => { "B B B " },
            0x17 => { "B B B|" },
            0x18 => { "M M B " },
            0x19 => { "M M B|" },
            0x1a => { return Err(DecodeError::BadBundle) },
            0x1b => { return Err(DecodeError::BadBundle) },
            0x1c => { "M F B " },
            0x1d => { "M F B|" },
            0x1e => { return Err(DecodeError::BadBundle) },
            0x1f => { return Err(DecodeError::BadBundle) },
            _ => { unreachable!(); }
        };
        eprintln!("bundle tag: {}", bundle_desc);

        // from here, `itanium-architecture-vol-1-2-3-4-reference-set-manual.pdf` volume 3 is
        // remaining necessary  details
        Ok(())
    }
}
