use yaxpeax_arch::{Arch, AddressDiff, Decoder, LengthedInstruction, NoColors, YaxColors};
use yaxpeax_arch::AddressBase;
use bitvec::prelude::*;

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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Opcode {
    // TODO; maybe don't actually do this one huh
    Purple,

    Addp4,
    Adds,
    Addl,
    Add,
    AddPlusOne,
    Sub,
    SubMinusOne,
    SubImm,
    AndImm,
    AndcmImm,
    OrImm,
    XorImm,
    And,
    Andcm,
    Or,
    Xor,

    Ptc_l,
    Probe_w_imm,
    Mov_to_psr_um,
    Probe_w,
    Ptc_g,
    Thash,
    Mov_m,
    Ptc_ga,
    Ttag,
    Ptr_d,
    Mov_to_cr,
    Ptr_i,
    Mov_to_psr_l,
    Itr_d,
    Tpa,
    Itc_d,
    Itr_i,
    Tak,
    Itc_i,
    Chk_s_m_int,
    Chk_s_fp,
    Alloc,
    Ld1,
    Ld2,
    Ld4,
    Ld8,
    Ld1_s,
    Ld2_s,
    Ld4_s,
    Ld8_s,
    Ld1_a,
    Ld2_a,
    Ld4_a,
    Ld8_a,
    Ld1_sa,
    Ld2_sa,
    Ld4_sa,
    Ld8_sa,
    Ld1_bias,
    Ld2_bias,
    Ld4_bias,
    Ld8_bias,
    Ld1_acq,
    Ld2_acq,
    Ld4_acq,
    Ld8_acq,
    Ld8_fill,
    Ld1_c_clr,
    Ld2_c_clr,
    Ld4_c_clr,
    Ld8_c_clr,
    Ld1_c_nc,
    Ld2_c_nc,
    Ld4_c_nc,
    Ld8_c_nc,
    Ld1_c_clr_acq,
    Ld2_c_clr_acq,
    Ld4_c_clr_acq,
    Ld8_c_clr_acq,
    St1,
    St2,
    St4,
    St8,
    St1_rel,
    St2_rel,
    St4_rel,
    St8_rel,
    St8_spill,
    Mov_to_pmd,
    Mov_from_pmd,
    Mov_from_psr,
    Mov_from_cpuid,
    Probe_r_imm,
    Probe_r,
    Cmpxchg1_acq,
    Cmpxchg2_acq,
    Cmpxchg4_acq,
    Cmpxchg8_acq,
    Cmpxchg1_rel,
    Cmpxchg2_rel,
    Cmpxchg4_rel,
    Cmpxchg8_rel,
    Xchg1,
    Xchg2,
    Xchg4,
    Xchg8,
    Fetchadd4_acq,
    Fetchadd8_acq,
    Fetchadd4_rel,
    Fetchadd8_rel,
    Getf_sig,
    Getf_exp,
    Getf_s,
    Getf_d,
    Cmp8xchg16_acq,
    Cmp8xchg16_rel,
    Ld16,
    Ld16_acq,
    St16,
    St16_rel,
    Ldfe,
    Ldf8,
    Ldfs,
    Ldfd,
    Ldfe_s,
    Ldf8_s,
    Ldfs_s,
    Ldfd_s,
    Ldfe_a,
    Ldf8_a,
    Ldfs_a,
    Ldfd_a,
    Ldfe_sa,
    Ldf8_sa,
    Ldfs_sa,
    Ldfd_sa,
    Ldf_fill,
    Ldfe_c_clr,
    Ldf8_c_clr,
    Ldfs_c_clr,
    Ldfd_c_clr,
    Ldfp8_c_clr,
    Ldfps_c_clr,
    Ldfpd_c_clr,
    Ldfp8_c_nc,
    Ldfps_c_nc,
    Ldfpd_c_nc,
    Ldfp8,
    Ldfps_a,
    Ldfps_sa,
    Invala,
    Fwb,
    Srlz_d,
    Srlz_i,
    Invala_e_int,
    Mf,
    Invala_e_fp,
    Mf_a,
    Sync_i,
    Sum,
    Rum,
    Ssm,
    Rsm,
    Loadrs,
    Flushrs,
    Hint_m,
    Nop_m,
    Chk_a_nc_int,
    Chk_a_clr_int,
    Chk_a_nc_fp,
    Chk_a_clr_fp,
    Mov_to_rr,
    Mov_from_rr,
    Fc,
    Mov_to_dbr,
    Mov_from_dbr,
    Mov_from_psr_um,
    Probe_rw_fault_imm,
    Mov_to_ibr,
    Mov_from_ibr,
    Mov_m_from_ar,
    Probe_r_fault_imm,
    Mov_to_pkr,
    Mov_fom_pkr,
    Probe_w_fault_imm,
    Mov_to_pmc,
    Mov_from_pmc,
    Mov_from_cr,
    Ptc_e,
    Ldfp_a,
    Ldfp_sa,
    Ldfe_c_nc,
    Ldf8_c_nc,
    Ldfs_c_nc,
    Ldfd_c_nc,
    Lfetch,
    Lfetch_excl,
    Lfetch_fault,
    Lfetch_fault_excl,
    Stfe,
    Stf8,
    Stfs,
    Stfd,
    Stf_spill,

    Shladd,
    Shladdp4,

    Padd1,
    Padd1_sss,
    Padd1_uuu,
    Padd1_uus,
    Psub1,
    Psub1_sss,
    Psub1_uuu,
    Psub1_uus,

    Ldfps,
    Ldfpd,
    Ldfp8_s,
    Ldfps_s,
    Ldfpd_s,
    Ldfp8_a,
    Ldfp8_a,
    Ldfpd_a,
    Ldfp8_sa,
    Ldfp8_sa,
    Ldfpd_sa,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {}
impl yaxpeax_arch::LengthedInstruction for Instruction {
    type Unit = yaxpeax_arch::AddressDiff<u64>;
    fn len(&self) -> Self::Unit { AddressDiff::from_const(16) }
    fn min_size() -> Self::Unit { AddressDiff::from_const(16) }
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

#[derive(Debug)]
pub enum Register {
    Application,
    Branch,
    Control,
    CpuId,
    DataBreakpoint,
    InstructionBreakpoint,
    DataTLBCache,
    DataTLBRegister,
    FloatingPoint,
    General,
    InstructionTLBCache,
    InstructionTLBRegister,
    ProtectionKey,
    PerformanceMonitorConfiguration,
    PerformanceMonitorData,
    Predicate,
    Region,
}
impl Decoder<Instruction> for InstDecoder {
    type Error = DecodeError;

    fn decode_into<T: IntoIterator<Item=u8>>(&self, inst: &mut Instruction, bytes: T) -> Result<(), Self::Error> {
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        enum InstructionType {
            A,
            I,
            M,
            F,
            B,
            LX,
        }

        let mut bytes_iter = bytes.into_iter();
        let mut instruction_bytes = bitarr![Lsb0, u8; 0u8; 128];
        for i in 0..0u64.wrapping_offset(Instruction::min_size()).to_linear() {
            instruction_bytes[(i * 8)..(i * 8 + 8)].store(bytes_iter.next().ok_or(DecodeError::ExhaustedInput)?);
        }
//        let instruction_bits = instruction_bytes.view_bits::<Lsb0>();
        let bundle_tag = instruction_bytes[0..5].load::<u8>();
        eprintln!("{:?}", instruction_bytes);
        let instruction_words = [
            &instruction_bytes[87..128],
            &instruction_bytes[46..87],
            &instruction_bytes[5..46],
        ];
        println!("bundle tag is now {:#05b}", bundle_tag);
        type BundleDesc = ([InstructionType; 3], u8); // u8 is a bitmap of which instructions are followed by stops.
        const BUNDLE_TAGS: [Option<BundleDesc>; 32] = [
            Some(([InstructionType::M, InstructionType::I, InstructionType::I], 0b000)),
            Some(([InstructionType::M, InstructionType::I, InstructionType::I], 0b001)),
            Some(([InstructionType::M, InstructionType::I, InstructionType::I], 0b010)),
            Some(([InstructionType::M, InstructionType::I, InstructionType::I], 0b011)),
            Some(([InstructionType::M, InstructionType::LX, InstructionType::LX], 0b000)),
            Some(([InstructionType::M, InstructionType::LX, InstructionType::LX], 0b010)),
            None,
            None,
            Some(([InstructionType::M, InstructionType::M, InstructionType::I], 0b000)),
            Some(([InstructionType::M, InstructionType::M, InstructionType::I], 0b001)),
            Some(([InstructionType::M, InstructionType::M, InstructionType::I], 0b100)),
            Some(([InstructionType::M, InstructionType::M, InstructionType::I], 0b101)),
            Some(([InstructionType::M, InstructionType::F, InstructionType::I], 0b000)),
            Some(([InstructionType::M, InstructionType::F, InstructionType::I], 0b001)),
            Some(([InstructionType::M, InstructionType::M, InstructionType::F], 0b000)),
            Some(([InstructionType::M, InstructionType::M, InstructionType::F], 0b001)),
            Some(([InstructionType::M, InstructionType::I, InstructionType::B], 0b000)),
            Some(([InstructionType::M, InstructionType::I, InstructionType::B], 0b001)),
            Some(([InstructionType::M, InstructionType::B, InstructionType::B], 0b000)),
            Some(([InstructionType::M, InstructionType::B, InstructionType::B], 0b001)),
            None,
            None,
            Some(([InstructionType::B, InstructionType::B, InstructionType::B], 0b000)),
            Some(([InstructionType::B, InstructionType::B, InstructionType::B], 0b001)),
            Some(([InstructionType::M, InstructionType::M, InstructionType::B], 0b000)),
            Some(([InstructionType::M, InstructionType::M, InstructionType::B], 0b001)),
            None,
            None,
            Some(([InstructionType::M, InstructionType::F, InstructionType::B], 0b000)),
            Some(([InstructionType::M, InstructionType::F, InstructionType::B], 0b001)),
            None,
            None,
        ];
        let (instruction_types, stop_mask) = BUNDLE_TAGS[bundle_tag as usize].ok_or(DecodeError::BadBundle)?;
        eprintln!("bundle types: {:?}, stop_mask: {:#b}", instruction_types, stop_mask);

        fn decode_instruction(word: &BitSlice<Lsb0, u8>, ty: InstructionType) -> Instruction {
            eprintln!("as slice: {:?}", word);
            let tag = word[37..41].load::<u8>();
            eprintln!("tag: {:#x}", tag);

            let ty = if tag >= 8 && (ty == InstructionType::M || ty == InstructionType::I) {
                InstructionType::A
            } else {
                ty
            };

            match ty {
                InstructionType::A => {
                    let (opcode, operand_encoding) = get_a_opcode_and_encoding(tag, word);
                    read_a_operands(operand_encoding, word);
                    panic!("aaa");
                }
                InstructionType::M => {
                    let (opcode, operand_encoding) = get_m_opcode_and_encoding(tag, word);
                    read_m_operands(operand_encoding, word);
                    panic!("mmm");
                    match tag {
                        4 => {
                            let x = word[27];
                            let m = word[36];
                            println!("x, m {}, {}", x, m);
                            if x == false && m == true {
                                let x6l = word[30..32].load::<u8>();
                                let x6u = word[32..36].load::<u8>();
                                println!("x6l, x6u {}, {}", x6l, x6u);
                                if x6u == 5 && x6l == 3 {
                                    panic!("ld8.acq");
                                }
                            } else {
                                panic!("b");
                            }
                        },
                        _ => {
                        }
                    }
                    Instruction {}
                }
                _ => {
                    Instruction {}
                }
            }
        }

        for (word, ty) in instruction_words.iter().zip(instruction_types.iter().cloned()) {
            let instruction = decode_instruction(word, ty);
            eprintln!("{:?}/{:#046b}: {:?}", ty, word, instruction);
        }

        // from here, `itanium-architecture-vol-1-2-3-4-reference-set-manual.pdf` volume 3 is
        // remaining necessary  details
        Ok(())
    }
}

fn read_m_operands(encoding: OperandEncodingM, word: &BitSlice<Lsb0, u8>) {
    use OperandEncodingM::*;
    match encoding {
        M1 => {
            let r1 = word[6..13].load::<u8>();
            let _ = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let hint = word[28..30].load::<u8>();
            eprintln!("m1 operands, r1={}, r3={}, hint={}", r1, r3, hint);
        },
        M2 => {
            let r1 = word[6..13].load::<u8>();
            let r2 = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let hint = word[28..30].load::<u8>();
            eprintln!("m2 operands, r1={}, r2={}, r3={}, hint={}", r1, r2, r3, hint);
        },
        M6 => {
            let _ = word[6..13].load::<u8>();
            let r2 = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let hint = word[28..30].load::<u8>();
            eprintln!("m6 operands, r2={}, r3={}, hint={}", r2, r3, hint);
        }
        M9 => {
            let f1 = word[6..13].load::<u8>();
            let _ = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let hint = word[28..30].load::<u8>();
            eprintln!("m9 operands, f1={}, r3={}, hint={}", f1, r3, hint);
        }
        M10 => {
            let f1 = word[6..13].load::<u8>();
            let r2 = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let hint = word[28..30].load::<u8>();
            eprintln!("m10 operands, f1={}, r2={}, r3={}, hint={}", f1, r2, r3, hint);
        }
        M11 => {
            let f1 = word[6..13].load::<u8>();
            let f2 = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let hint = word[28..30].load::<u8>();
            eprintln!("m11 operands, f1={}, f2={}, r3={}, hint={}", f1, f2, r3, hint);
        }
        M12 => {
            let f1 = word[6..13].load::<u8>();
            let f2 = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let hint = word[28..30].load::<u8>();
            eprintln!("m12 operands, f1={}, f2={}, r3={}, hint={}", f1, f2, r3, hint);
        }
        M13 => {
            let _ = word[6..13].load::<u8>();
            let f2 = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let hint = word[28..30].load::<u8>();
            eprintln!("m13 operands, f2={}, r3={}, hint={}", f2, r3, hint);
        }
        M14 => {
            let _ = word[6..13].load::<u8>();
            let r2 = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let hint = word[28..30].load::<u8>();
            eprintln!("m14 operands, r2={}, r3={}, hint={}", r2, r3, hint);
        }
        M16 => {
            let r1 = word[6..13].load::<u8>();
            let r2 = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let hint = word[28..30].load::<u8>();
            eprintln!("m16 operands, r1={}, r2={}, r3={}, hint={}", r1, r2, r3, hint);
        }
        other => {
            unimplemented!("unimplemented m operand encoding: {:?}", other);
        }
    }
}

fn read_a_operands(encoding: OperandEncodingA, word: &BitSlice<Lsb0, u8>) {
    use OperandEncodingA::*;
    match encoding {
        A1 => {
            let r1 = word[6..13].load::<u8>();
            let r2 = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            eprintln!("a1 operands, r1={}, r2={}, r3={}", r1, r2, r3);
        },
        A2 => {
            let r1 = word[6..13].load::<u8>();
            let r2 = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let ct = word[27..29].load::<u8>();
            eprintln!("a2 operands, r1={}, r2={}, r3={}, ct={}", r1, r2, r3, ct);
        },
        A3 => {
            let r1 = word[6..13].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let immb = word[13..20].load::<u8>();
            let s = word[36];
            let imm = (immb + ((s as u8) << 7)) as i8 as i32;
            eprintln!("a3 operands, r1={}, r3={}, imm={}", r1, r3, imm);
        },
        A4 => {
            let r1 = word[6..13].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let immb = word[13..20].load::<u16>();
            let immd = word[27..33].load::<u16>();
            let s = word[36];
            let imm = ((s as u16) << 12) + (immd << 7) + immb;
            let imm = (((imm as i16) << 2) >> 2) as i32;
            eprintln!("a4 operands, r1={}, r3={}, imm={}", r1, r3, imm);
        },
        A5 => {
            let r1 = word[6..13].load::<u8>();
            let r3 = word[20..22].load::<u8>();
            let immb = word[13..20].load::<u32>();
            let immc_d_s = word[22..37].load::<u32>();
            let imm = (immc_d_s << 7) + immb;
            let imm = ((imm as i32) << 10) >> 10;
            eprintln!("a5 operands, r1={}, r3={}, imm={}", r1, r3, imm);
        }
        A6 => {
            let p1 = word[6..12].load::<u8>();
            let r2 = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let p2 = word[27..33].load::<u8>();
            eprintln!("a6 operands, r2={}, r3={}, p1={}, p2={}", r2, r3, p1, p2);
        },
        A7 => {
            let p1 = word[6..12].load::<u8>();
            let r2 = word[13..20].load::<u8>();
            // TODO: what happens if this isn't zero?
            assert_eq!(r2, 0);
            let r3 = word[20..27].load::<u8>();
            let p2 = word[27..33].load::<u8>();
            eprintln!("a7 operands, r3={}, p1={}, p2={}", r3, p1, p2);
        },
        A8 => {
            let p1 = word[6..12].load::<u8>();
            let imm7b = word[13..20].load::<u8>();
            let s = word[36];
            let imm = (imm7b + (s as u8) << 7) as i8 as i32;
            let r3 = word[20..27].load::<u8>();
            let p2 = word[27..33].load::<u8>();
            eprintln!("a8 operands, imm={}, r3={}, p1={}, p2={}", imm, r3, p1, p2);
        },
        A9 => {
            let r1 = word[6..13].load::<u8>();
            let r2 = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            eprintln!("a9 operands, r1={}, r2={}, r3={}, ct={}", r1, r2, r3);
        },
        A10 => {
            let r1 = word[6..13].load::<u8>();
            let r2 = word[13..20].load::<u8>();
            let r3 = word[20..27].load::<u8>();
            let ct = word[27..29].load::<u8>();
            eprintln!("a10 operands, r1={}, r2={}, r3={}, ct={}", r1, r2, r3, ct);
        },
    }
}

fn get_m_opcode_and_encoding(tag: u8, word: &BitSlice<Lsb0, u8>) -> (Opcode, OperandEncodingM) {
    use Opcode::*;
    use OperandEncodingM::*;

    match tag {
        0 => {
            let x3 = word[33..36].load::<u8>();
            // `Table 4-42 Opcode 0 System/Memory Management 3-bit Opcode Extensions`
            if x3 == 0 {
                // `Table 4-43 System/Memory Management 4-bit+2-bit Ext`
                const TABLE4_43: [(Opcode, OperandEncodingM); 64] = [
                    (Break_m, M37), (Invala, M24), (Fwb, M24), (Srlz_d, M24),
                    // `1-bit Ext (Table 4-46)` is handled independently
                    (Purple, None), (Purple, None), (Purple, None), (Srlz_i, M24),
                    (Purple, None), (Invala_e_int, M26), (Mf, M24), (Purple, None),
                    (Purple, None), (Invala_e_fp, M27), (Mf_a, M24), (Sync_i, M24),
                    (Sum, M44), (Sum, M44), (Sum, M44), (Sum, M44),
                    (Rum, M44), (Rum, M44), (Rum, M44), (Rum, M44),
                    (Ssm, M44), (Ssm, M44), (Ssm, M44), (Ssm, M44),
                    (Rsm, M44), (Rsm, M44), (Rsm, M44), (Rsm, M44),
                    (Purple, None), (Purple, None), (Mov_m_to_ar_imm, M30), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Loadrs, M25), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Flushrs, M25), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                ];
                let index = word[27..33].load::<u8>();
                if index == 0b00001 {
                    // `1-bit Ext (Table 4-46)`
                    return if word[26] {
                        (Hint_m, M48)
                    } else {
                        (Nop_m, M48)
                    }
                }
                TABLE4_43[index as usize]
            } else {
                const TABLE4_42: [(Opcode, OperandEncodingM); 7] = [
                    (Purple, None),
                    (Purple, None),
                    (Purple, None),
                    (Chk_a_nc_int, M22),
                    (Chk_a_clr_int, M22),
                    (Chk_a_nc_fp, M23),
                    (Chk_a_clr_fp, M23),
                ];
                TABLE4_42[x3 as usize]
            }
        },
        1 => {
            let x3 = word[33..36].load::<u8>();
            // `Table 4-44 Opcode 1 System/Memory Management 3-bit Opcode Extensions`
            if x3 == 0 {
                unimplemented!("table 4-45");
                // `Table 4-45 System/Memory Management 6-bit Ext`
                const TABLE4_45: [(Opcode, OperandEncodingM); 64] = [
                    (Mov_to_rr, M42), (Mov_from_rr, M43), (Purple, None), (Fc, M28),
                    (Mov_to_dbr, M42), (Mov_from_dbr, M43), (Mov_from_psr_um, M36), (Probe_rw_fault_imm, M40),
                    (Mov_to_ibr, M42), (Mov_from_ibr, M43), (Mov_m_from_ar, M31), (Probe_r_fault_imm, M40),
                    (Mov_to_pkr, M43), (Mov_from_pkr, M43), (Purple, None), (Probe_w_fault_imm, M40),
                    (Mov_to_pmc, M42), (Mov_from_pmc, M43), (Mov_from_cr, M33), (Ptc_e, M47),
                    (Mov_to_pmd, M42), (Mov_from_pmd, M43), (Mov_from_psr, M36), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Mov_from_cpuid, M43), (Purple, None), (Purple, None),
                    (Purple, None), (Probe_r_imm, M39), (Purple, None), (Probe_r, M38),
                    (Ptc_l, M45), (Probe_w_imm, M39), (Mov_to_psr_um, M35), (Probe_w, M38),
                    (Ptc_g, M45), (Thash, M46), (Mov_m, M29), (Purple, None),
                    (Ptc_ga, M45), (Ttag, M46), (Purple, None), (Purple, None),
                    (Ptr_d, M45), (Purple, None), (Mov_to_cr, M32), (Purple, None),
                    (Ptr_i, M45), (Purple, None), (Mov_to_psr_l, M35), (Purple, None),
                    (Itr_d, M42), (Tpa, M46), (Itc_d, M41), (Purple, None),
                    (Itr_i, M42), (Tak, M46), (Itc_i, M41), (Purple, None),
                ];
                let index = word[27..33].load::<u8>();
                TABLE4_45[index as usize]
            } else {
                const TABLE4_44: [(Opcode, OperandEncodingM); 7] = [
                    (Chk_s_m_int, M20),
                    (Purple, None),
                    (Chk_s_fp, M21),
                    (Purple, None),
                    (Purple, None),
                    (Alloc, M34),
                    (Purple, None),
                ];
                TABLE4_44[x3 as usize]
            }
        },
        2 => { (Purple, None) },
        3 => { (Purple, None) },
        4 => {
            // `Table 4-28 Integer Load/Store/Semaphore/Get FR 1-bit Opcode Extensions`
            const TABLE4_28: [Option<&'static [(Opcode, OperandEncodingM); 64]>; 4] = [
                Some(&TABLE4_30),
                Some(&TABLE4_33),
                Some(&TABLE4_31),
                None
            ];

            // `Table 4-30 Integer Load/Store Opcode Extensions
            const TABLE4_30: [(Opcode, OperandEncodingM); 64] = [
                (Ld1, M2), (Ld2, M2), (Ld4, M2), (Ld8, M2),
                (Ld1_s, M2), (Ld2_s, M2), (Ld4_s, M2), (Ld8_s, M2),
                (Ld1_a, M2), (Ld2_a, M2), (Ld4_a, M2), (Ld8_a, M2),
                (Ld1_sa, M2), (Ld2_sa, M2), (Ld4_sa, M2), (Ld8_sa, M2),
                (Ld1_bias, M2), (Ld2_bias, M2), (Ld4_bias, M2), (Ld8_bias, M2),
                (Ld1_acq, M2), (Ld2_acq, M2), (Ld4_acq, M2), (Ld8_acq, M2),
                (Purple, None), (Purple, None), (Purple, None), (Ld8_fill, M2),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Ld1_c_clr, M2), (Ld2_c_clr, M2), (Ld4_c_clr, M2), (Ld8_c_clr, M2),
                (Ld1_c_nc, M2), (Ld2_c_nc, M2), (Ld4_c_nc, M2), (Ld8_c_nc, M2),
                (Ld1_c_clr_acq, M2), (Ld2_c_clr_acq, M2), (Ld4_c_clr_acq, M2), (Ld8_c_clr_acq, M2),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (St1, M6), (St2, M6), (St4, M6), (St8, M6),
                (St1_rel, M6), (St2_rel, M6), (St4_rel, M6), (St8_rel, M6),
                (Purple, None), (Purple, None), (Purple, None), (St8_spill, M6),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
            ];

            // `Table 4-31 Floating-point Load/Store/Lfetch +Imm Opcode Extensions`
            const TABLE4_31: [(Opcode, OperandEncodingM); 64] = [
                (Ld1, M2), (Ld2, M2), (Ld4, M2), (Ld8, M2),
                (Ld1_s, M2), (Ld2_s, M2), (Ld4_s, M2), (Ld8_s, M2),
                (Ld1_a, M2), (Ld2_a, M2), (Ld4_a, M2), (Ld8_a, M2),
                (Ld1_sa, M2), (Ld2_sa, M2), (Ld4_sa, M2), (Ld8_sa, M2),
                (Ld1_bias, M2), (Ld2_bias, M2), (Ld4_bias, M2), (Ld8_bias, M2),
                (Ld1_acq, M2), (Ld2_acq, M2), (Ld4_acq, M2), (Ld8_acq, M2),
                (Purple, None), (Purple, None), (Purple, None), (Ld8_fill, M2),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Ld1_c_clr, M2), (Ld2_c_clr, M2), (Ld4_c_clr, M2), (Ld8_c_clr, M2),
                (Ld1_c_nc, M2), (Ld2_c_nc, M2), (Ld4_c_nc, M2), (Ld8_c_nc, M2),
                (Ld1_c_clr_acq, M2), (Ld2_c_clr_acq, M2), (Ld4_c_clr_acq, M2), (Ld8_c_clr_acq, M2),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
            ];

            // `Table 4-33 Semaphore/Get FR/16-byte Opcode Extensions`
            const TABLE4_33: [(Opcode, OperandEncodingM); 64] = [
                (Cmpxchg1_acq, M16), (Cmpxchg2_acq, M16), (Cmpxchg4_acq, M16), (Cmpxchg8_acq, M16),
                (Cmpxchg1_rel, M16), (Cmpxchg2_rel, M16), (Cmpxchg4_rel, M16), (Cmpxchg8_rel, M16),
                (Xchg1, M16), (Xchg2, M16), (Xchg4, M16), (Xchg8, M16),
                (Purple, None), (Purple, None), (Purple, None), (Ld8_fill, M2),
                (Purple, None), (Purple, None), (Fetchadd4_acq, M17), (Fetchadd8_acq, M17),
                (Purple, None), (Purple, None), (Fetchadd4_rel, M17), (Fetchadd8_rel, M17),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Getf_sig, M19), (Getf_exp, M19), (Getf_s, M19), (Getf_d, M19),
                (Cmp8xchg16_acq, M16), (Purple, None), (Purple, None), (Purple, None),
                (Cmp8xchg16_rel, M16), (Purple, None), (Purple, None), (Purple, None),
                (Ld16, M2), (Purple, None), (Purple, None), (Purple, None),
                (Ld16_acq, M2), (Purple, None), (Purple, None), (Purple, None),
                (St16, M6), (Purple, None), (Purple, None), (Purple, None),
                (St16_rel, M6), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
            ];

            let index = ((word[36] as u8) << 1) + (word[27] as u8);
            if let Some(op_table) = TABLE4_28[index as usize] {
                op_table[word[30..36].load::<u8>() as usize]
            } else {
                (Purple, None)
            }
        },
        5 => {
            unimplemented!("table 4-32");
            // `Table 4-32 Integer Load/Store +Imm Opcode Extensions`
            const TABLE4_32: [(Opcode, OperandEncodingM); 64] = [
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
            ];

            TABLE4_32[word[30..36].load::<u8>() as usize]
        },
        6 => {
            // `Table 4-29 Floating-point Load/Store/Load Pair/Set FR 1-bit Opcode Extensions`
            const TABLE4_29: [&'static [(Opcode, OperandEncodingM); 64]; 4] = [
                &TABLE4_34,
                &TABLE4_37,
                &TABLE4_35,
                &TABLE4_38,
            ];

            // `Table 4-34 Floating-point Load/Store/Lfetch Opcode Extensions`
            const TABLE4_34: [(Opcode, OperandEncodingM); 64] = [
                (Ldfe, M9), (Ldf8, M9), (Ldfs, M9), (Ldfd, M9),
                (Ldfe_s, M9), (Ldf8_s, M9), (Ldfs_s, M9), (Ldfd_s, M9),
                (Ldfe_a, M9), (Ldf8_a, M9), (Ldfp_a, M9), (Ldfd_a, M9),
                (Ldfe_sa, M9), (Ldf8_sa, M9), (Ldfp_sa, M9), (Ldfd_sa, M9),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Ldf_fill, M9),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Ldfe_c_clr, M9), (Ldf8_c_clr, M9), (Ldfs_c_clr, M9), (Ldfd_c_clr, M9),
                (Ldfe_c_nc, M9), (Ldf8_c_nc, M9), (Ldfs_c_nc, M9), (Ldfd_c_nc, M9),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Lfetch, M18), (Lfetch_excl, M18), (Lfetch_fault, M18), (Lfetch_fault_excl, M18),
                (Stfe, M13), (Stf8, M13), (Stfs, M13), (Stfd, M13),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Stf_spill, M13),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
            ];

            // `Table 4-35 Floating-point Load/Lfetch +Reg Opcode Extensions`
            const TABLE4_35: [(Opcode, OperandEncodingM); 64] = [
                (Ldfe, M7), (Ldf8, M7), (Ldfs, M7), (Ldfd, M7),
                (Ldfe_s, M7), (Ldf8_s, M7), (Ldfs_s, M7), (Ldfd_s, M7),
                (Ldfe_a, M7), (Ldf8_a, M7), (Ldfp_a, M7), (Ldfd_a, M7),
                (Ldfe_sa, M7), (Ldf8_sa, M7), (Ldfp_sa, M7), (Ldfd_sa, M7),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Ldf_fill, M7),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Ldfe_c_clr, M7), (Ldf8_c_clr, M7), (Ldfs_c_clr, M7), (Ldfd_c_clr, M7),
                (Ldfe_c_nc, M7), (Ldf8_c_nc, M7), (Ldfs_c_nc, M7), (Ldfd_c_nc, M7),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Lfetch, M20), (Lfetch_excl, M20), (Lfetch_fault, M20), (Lfetch_fault_excl, M20),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
            ];

            // `Table 4-37 Floating-point Load Pair/Set FR Opcode Extensions`
            const TABLE4_37: [(Opcode, OperandEncodingM); 64] = [
                (Purple, None), (Ldfp8, M11), (Ldfps, M11), (Ldfpd, M11),
                (Purple, None), (Ldfp8_s, M11), (Ldfps_s, M11), (Ldfpd_s, M11),
                (Purple, None), (Ldfp8_a, M11), (Ldfps_a, M11), (Ldfpd_a, M11),
                (Purple, None), (Ldfp8_sa, M11), (Ldfps_sa, M11), (Ldfpd_sa, M11),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Setf_sig, M18), (Setf_exp, M18), (Setf_s, M18), (Setf_d, M18),
                (Purple, None), (Ldfp8_c_clr, M11), (Ldfps_c_clr, M11), (Ldfpd_c_clr, M11),
                (Purple, None), (Ldfp8_c_nc, M11), (Ldfps_c_nc, M11), (Ldfpd_c_nc, M11),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
            ];

            // `Table 4-38 Floating-point Load Pair +Imm Opcode Extensions`
            const TABLE4_38: [(Opcode, OperandEncodingM); 64] = [
                (Purple, None), (Ldfp8, M12), (Ldfps, M12), (Ldfpd, M12),
                (Purple, None), (Ldfp8_s, M12), (Ldfps_s, M12), (Ldfpd_s, M12),
                (Purple, None), (Ldfp8_a, M12), (Ldfps_a, M12), (Ldfpd_a, M12),
                (Purple, None), (Ldfp8_sa, M12), (Ldfps_sa, M12), (Ldfpd_sa, M12),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Ldfp8_c_clr, M12), (Ldfps_c_clr, M12), (Ldfpd_c_clr, M12),
                (Purple, None), (Ldfp8_c_nc, M12), (Ldfps_c_nc, M12), (Ldfpd_c_nc, M12),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
            ];

            let index = ((word[36] as u8) << 1) + (word[27] as u8);
            let op_table = TABLE4_29[index as usize];
            op_table[word[30..36].load::<u8>() as usize]
        },
        7 => {
            // `Table 4-36 Floating-point Load/Store/Lfetch +Imm Opcode Extensions`
            const TABLE4_36: [(Opcode, OperandEncodingM); 64] = [
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
            ];

            TABLE4_36[word[30..36].load::<u8>() as usize]
        }
        _ => {
            unreachable!("m major op > 7 are a-type instructions");
        }
    }
}

fn get_a_opcode_and_encoding(tag: u8, word: &BitSlice<Lsb0, u8>) -> (Opcode, OperandEncodingA) {
    use Opcode::*;
    use OperandEncodingA::*;

    match tag {
        8 => {
            let x2a = word[34..36].load::<u8>();
            eprintln!("zb: {}", word[33]);
            if word[33] && x2a != 1 {
                // purple (table 4-8 ve=1, x2a = 0, 2, or 3)
                return (Purple, None);
            }

            // table 4-8: `Integer ALU 2-bit+1-bit Opcode Extensions`
            match x2a {
                0 => {
                    const TABLE4_9: [(Opcode, OperandEncodingA); 64] = [
                        (Add, A1), (AddPlusOne, A1), (Purple, None), (Purple, None),
                        (SubMinusOne, A1), (Sub, A1), (Purple, None), (Purple, None),
                        (Addp4, A1), (Purple, None), (Purple, None), (Purple, None),
                        (And, A1), (Andcm, A1), (Or, A1), (Xor, A1),
                        (Shladd, A2), (Shladd, A2), (Shladd, A2), (Shladd, A2),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Shladdp4, A2), (Shladdp4, A2), (Shladdp4, A2), (Shladdp4, A2),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (SubImm, A3), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (AndImm, A3), (AndcmImm, A3), (OrImm, A3), (XorImm, A3),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    ];
                    TABLE4_9[word[27..33].load::<u8>() as usize]
                }
                1 => {
                    // `Multimedia ALU Table 4-12`
                    const TABLE4_12: [Option<&'static [(Opcode, OperandEncodingA); 64]>; 4] = [
                        Some(&TABLE4_13),
                        Some(&TABLE4_14),
                        Some(&TABLE4_15),
                        None
                    ];

                    const TABLE4_13: [(Opcode, OperandEncodingA); 64] = [
                        (Padd1, A9), (Padd1_sss, A9), (Padd1_uuu, A9), (Padd1_uus, A9),
                        (Psub1, A9), (Psub1_sss, A9), (Psub1_uuu, A9), (Psub1_uus, A9),
                        (Purple, None), (Purple, None), (Pavg1, A9), (Pavg1_raz, A9),
                        (Purple, None), (Purple, None), (Pavgsub1, A9), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Pcmp1_eq, A9), (Pcmp1_gt, A9), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    ];

                    const TABLE4_14: [(Opcode, OperandEncodingA); 64] = [
                        (Padd2, A9), (Padd2_sss, A9), (Padd2_uuu, A9), (Padd2_uus, A9),
                        (Psub2, A9), (Psub2_sss, A9), (Psub2_uuu, A9), (Psub2_uus, A9),
                        (Purple, None), (Purple, None), (Pavg2,g A9), (Pavg2_raz, A9),
                        (Purple, None), (Purple, None), (Pavgsub2,g A9), (Purple, None),
                        (Pshladd2, A10), (Pshladd2, A10), (Pshladd2, A10), (Pshladd2, A10),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Pshradd2, A10), (Pshradd2, A10), (Pshradd2, A10), (Pshradd2, A10),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Pcmp2_eq, A9), (Pcmp2_gt, A9), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    ];

                    const TABLE4_15: [(Opcode, OperandEncodingA); 64] = [
                        (Padd4, A9), (Purple, None), (Purple, None), (Purple, None),
                        (Psub4, A9), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Pcmp4_eq, A9), (Pcmp4_gt, A9), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                        (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    ];

                    let index = ((word[36] as u8) << 1) + (word[33] as u8);
                    if let Some(alu_table) = TABLE4_12[index as usize] {
                        alu_table[word[27..33].load::<u8>() as usize]
                    } else {
                        (Purple, None)
                    }
                },
                2 => {
                    (Opcode::Adds, OperandEncodingA::A4)
                }
                3 => {
                    (Opcode::Addp4, OperandEncodingA::A4)
                }
            }
        }
        9 => (Addl, A5),
        0xa => (Purple, None),
        0xb => (Purple, None),
        0xc => {
            let x2 = word[34..36].load::<u8>();
            if x2 > 1 {
                // `Table 4-11 Integer Compare Immediate Opcode Extensions`
                let encoding = A8;
            } else {
                // `Table 4-10 Integer Compare Opcode Extensions`
                let encoding = if word[36] { A7 } else { A6 };
            }
            unimplemented!("type a, tag c");
        }
        0xd => {
            let x2 = word[34..36].load::<u8>();
            if x2 > 1 {
                // `Table 4-11 Integer Compare Immediate Opcode Extensions`
                let encoding = A8;
            } else {
                // `Table 4-10 Integer Compare Opcode Extensions`
                let encoding = if word[36] { A7 } else { A6 };
            }
            unimplemented!("type a, tag d");
        }
        0xe => {
            let x2 = word[34..36].load::<u8>();
            if x2 > 1 {
                // `Table 4-11 Integer Compare Immediate Opcode Extensions`
                let encoding = A8;
            } else {
                // `Table 4-10 Integer Compare Opcode Extensions`
                let encoding = if word[36] { A7 } else { A6 };
            }
            unimplemented!("type a, tag e");
        }
        0xf => (Purple, None),
        _ => {
            unreachable!("a-type major op < 8 are i-type or m-type instructions");
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum OperandEncodingA {
    None,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    A10,
}

#[derive(Copy, Clone, Debug)]
enum OperandEncodingI {
    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
    I8,
    I9,
    I10,
    I11,
    I12,
    I13,
    I14,
    I15,
    I16,
    I17,
    I18,
    I19,
    I20,
    I21,
    I22,
    I23,
    I24,
    I25,
    I26,
    I27,
    I28,
    I29,
    I30,
}

#[derive(Copy, Clone, Debug)]
enum OperandEncodingM {
    None,
    M1,
    M2,
    M3,
    M4,
    M5,
    M6,
    M7,
    M8,
    M9,
    M10,
    M11,
    M12,
    M13,
    M14,
    M15,
    M16,
    M17,
    M18,
    M19,
    M20,
    M21,
    M22,
    M23,
    M24,
    M25,
    M26,
    M27,
    M28,
    M29,
    M30,
    M31,
    M32,
    M33,
    M34,
    M35,
    M36,
    M37,
    M38,
    M39,
    M40,
    M41,
    M42,
    M43,
    M44,
    M45,
    M46,
    M47,
    M48,
}

#[derive(Copy, Clone, Debug)]
enum OperandEncodingB {
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
}

#[derive(Copy, Clone, Debug)]
enum OperandEncodingF {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
}

#[derive(Copy, Clone, Debug)]
enum OperandEncodingX {
    X1,
    X2,
    X3,
    X4,
    X5,
}

