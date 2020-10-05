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
#[allow(non_camel_case_types)]
pub enum Opcode {
    // TODO: what kind of no-op/undefined are these exactly
    Purple,
    Cyan,
    Brown,
    White,

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
    Break_m,
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

    Ldfp8,
    Ldfps,
    Ldfpd,
    Ldfp8_s,
    Ldfps_s,
    Ldfpd_s,
    Ldfp8_a,
    Ldfps_a,
    Ldfpd_a,
    Ldfp8_sa,
    Ldfps_sa,
    Ldfpd_sa,

    Mov_m_to_ar_imm,
    Mov_from_pkr,
    Setf_sig,
    Setf_exp,
    Setf_s,
    Setf_d,
    Pavg1,
    Pavg1_raz,
    Pavgsub1,
    Pcmp1_eq,
    Pcmp1_gt,
    Padd2,
    Padd2_sss,
    Padd2_uuu,
    Padd2_uus,
    Psub2,
    Psub2_sss,
    Psub2_uuu,
    Psub2_uus,
    Pavg2,
    Pavg2_raz,
    Pavgsub2,
    Pshladd2,
    Pshradd2,
    Pcmp2_eq,
    Pcmp2_gt,
    Padd4,
    Psub4,
    Pcmp4_eq,
    Pcmp4_gt,
    Hint_x,
    Nop_x,
    Movl,
    Brl_cond_bwh_ph_dh,
    Brl_call_bwh_ph_dh,
    Br_call_bwh_ph_dh,
    Brp_ipwh_ih,
    Break_x,
    Break_i,
    Zxt1,
    Mov_from_ip,
    Zxt2,
    Mov_from_b,
    Zxt4,
    Mov_i_from_ar,
    Sxt1,
    Sxt2,
    Sxt4,
    Czx1_l,
    Czx2_l,
    Mov_i_to_ar_imm,
    Mov_i_to_ar,
    Czx1_r,
    Czx2_r,
    Hint_i,
    Nop_i,
    Chk_s_i_int,
    Mov_to_pr_rot_imm,
    Mov_to_pr,
    Mov_from_pr,
    Mov_to_b,
    Dep,
    Tbit_z,
    Tnat_z,
    Tbit_z_unc,
    Tnat_z_unc,
    Tbit_z_and,
    Tnat_z_and,
    Tbit_nz_and,
    Tnat_nz_and,
    Tbit_z_or,
    Tnat_z_or,
    Tbit_nz_or,
    Tnat_nz_or,
    Tbit_z_or_andcm,
    Tnat_z_or_andcm,
    Tbit_nz_or_andcm,
    Tnat_nz_or_andcm,
    Tf_z,
    Tf_z_nc,
    Tf_z_and,
    Tf_nz_and,
    Tf_z_or,
    Tf_nz_or,
    Tf_z_or_andcm,
    Tf_nz_or_andcm,
    Dep_z_imm,
    Dep_imm,
    Dep_z,
    Extr,
    Shrp,
    Extr_u,
    Pmin1_u,
    Unpack1_h,
    Pmax1_u,
    Unpack1_l,
    Mix1_r,
    Mix1_l,
    Psad1,
    Mux1,
    Pshr2_u_var,
    Pmpyshr2_u,
    Pshr2_var,
    Pmpyshr2,
    Pshl1_var,
    Pshr2_u_fixed,
    Pshr2_fixed,
    Popcnt,
    Clz,
    Pack2_uss,
    Pack2_sss,
    Pmin2,
    Unpack2_h,
    Unpack2_l,
    Pmax2,
    Mix2_r,
    Mix2_l,
    Pmpy2_r,
    Pmpy2_l,
    Pshl2_fixed,
    Mux2,
    Pshr4_u_var,
    Pshr4_var,
    Pshl4_var,
    Mpy4,
    Mpyshl4,
    Pshr4_u_fixed,
    Pshr4_fixed,
    Pack4_sss,
    Unpack4_h,
    Unpack4_l,
    Mix4_r,
    Mix4_l,
    Pshl4_fixed,
    Shr_u_var,
    Shr_var,
    Shl_var,

    Break_b,
    Cover,
    Clrrb,
    Clrrb_pr,
    Rfi,
    Bsw_0,
    Bsw_1,
    Epc,
    Vmsw_0,
    Vmsw_1,
    Br_cond,
    Br_ia,
    Br_ret,

    Nop_b,
    Hint_b,
    Brp,
    Brp_ret,

    Br_wexit,
    Br_wtop,
    Br_cloop,
    Br_cexit,
    Br_ctop,

    Frcpa,
    Frsqta,
    Break_f,
    Fsetc,
    Fclrf,
    Fchkf,
    Fmerge_s,
    Fmerge_ns,
    Fmerge_se,

    Fmin,
    Fmax,
    Famin,
    Famax,
    Fcvt_fx,
    Fcvt_fxu,
    Fcvt_fx_trunc,
    Fcvt_fxu_trunc,
    Fcvt_xf,
    Fpack,
    Fand,
    Fandcm,
    For,
    Fxor,

    Fswap,
    Fswap_nl,
    Fswap_nr,
    Fmix_lr,
    Fmix_r,
    Fmix_l,

    Fsxt_r,
    Fsxt_l,

    Hint_f,
    Nop_f,

    Fprcpa,
    Fprsqrta,
    Fpmerge_s,
    Fpmerge_ns,
    Fpmerge_se,

    Fpmin,
    Fpmax,
    Fpamin,
    Fpamax,
    Fpcvt_fx,
    Fpcvt_fxu,
    Fpcvt_fx_trunc,
    Fpcvt_fxu_trunc,
    Fcmp_eq,
    Fcmp_lt,
    Fcmp_le,
    Fcmp_unord,
    Fcmp_eq_unc,
    Fcmp_lt_unc,
    Fcmp_le_unc,
    Fcmp_unord_unc,
    Fclass_m_unc,
    Fclass_m,
    Fma_s_sf,
    Fma_sf,
    Fpma_sf,
    Fma_d_sf,
    Fms_s_sf,
    Fms_sf,
    Fpms_sf,
    Fms_d_sf,
    Fnma_s_sf,
    Fnma_sf,
    Fpnma_sf,
    Fnma_d_sf,
    Xma_l,
    Xma_hu,
    Xma_h,
    Fselect,
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
                InstructionType::I => {
                    let (opcode, operand_encoding) = get_i_opcode_and_encoding(tag, word);
                    read_i_operands(operand_encoding, word);
                    panic!("i");
                },
                InstructionType::F => {
                    let (opcode, operand_encoding) = get_f_opcode_and_encoding(tag, word);
                    read_f_operands(operand_encoding, word);
                    panic!("f");
                },
                InstructionType::B => {
                    let (opcode, operand_encoding) = get_b_opcode_and_encoding(tag, word);
                    read_b_operands(operand_encoding, word);
                    panic!("b");
                },
                InstructionType::LX => {
                    let (opcode, operand_encoding) = get_lx_opcode_and_encoding(tag, word);
                    read_lx_operands(operand_encoding, word);
                    panic!("lx");
                },
                InstructionType::A => {
                    let (opcode, operand_encoding) = get_a_opcode_and_encoding(tag, word);
                    eprintln!("A {:?}/{:?}", opcode, operand_encoding);
                    read_a_operands(operand_encoding, word);
                    Instruction {}
                }
                InstructionType::M => {
                    let (opcode, operand_encoding) = get_m_opcode_and_encoding(tag, word);
                    eprintln!("M({}) {:?}/{:?}", tag, opcode, operand_encoding);
                    read_m_operands(operand_encoding, word);
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

fn read_lx_operands(encoding: OperandEncodingX, word: &BitSlice<Lsb0, u8>) {
}
fn read_b_operands(encoding: OperandEncodingB, word: &BitSlice<Lsb0, u8>) {
}
fn read_f_operands(encoding: OperandEncodingF, word: &BitSlice<Lsb0, u8>) {
}
fn read_i_operands(encoding: OperandEncodingI, word: &BitSlice<Lsb0, u8>) {
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
        None => { unreachable!("none operand encoding"); }
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
            eprintln!("a9 operands, r1={}, r2={}, r3={}", r1, r2, r3);
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

fn get_lx_opcode_and_encoding(tag: u8, word: &BitSlice<Lsb0, u8>) -> (Opcode, OperandEncodingX) {
    use Opcode::*;
    use OperandEncodingX::*;

    match tag {
        0x0 => {
            let x3 = word[33..36].load::<u8>();
            if x3 == 0 {
                // `Table 4-70 Misc X-Unit 6-bit Opcode Extensions`
                let x6 = word[27..33].load::<u8>();
                if x6 == 0 {
                    (Break_x, X1)
                } else if x6 == 1 {
                    // `1-bit Ext (Table 4-73)`
                    if word[26] {
                        (Hint_x, X5)
                    } else {
                        (Nop_x, X5)
                    }
                } else {
                    (Purple, None)
                }
            } else {
                (Purple, None)
            }
        },
        0x1 => { (Purple, None) },
        0x2 => { (Purple, None) },
        0x3 => { (Purple, None) },
        0x4 => { (Purple, None) },
        0x5 => { (Purple, None) },
        0x6 => { (Movl, X2) },
        0x7 => { (Purple, None) },
        0x8 => { (Cyan, None) },
        0x9 => { (Cyan, None) },
        0xa => { (Cyan, None) },
        0xb => { (Cyan, None) },
        0xc => {
            // p, wh, d, described in tables 4-51, 4-52, 4-54
            (Brl_cond_bwh_ph_dh, X3)
        },
        0xd => {
            // p, wh, d, described in tables 4-51, 4-52, 4-54
            (Brl_call_bwh_ph_dh, X4)
        },
        0xe => { (Cyan, None) },
        0xf => { (Cyan, None) },
        _ => { unreachable!() },
    }
}

fn get_b_opcode_and_encoding(tag: u8, word: &BitSlice<Lsb0, u8>) -> (Opcode, OperandEncodingB) {
    use Opcode::*;
    use OperandEncodingB::*;

    match tag {
        0x0 => {
            // `Table 4-48 Indirect/Miscellaneous Branch Opcode Extensions`
            const TABLE4_48: [(Opcode, OperandEncodingB); 64] = [
                (Break_b, B9), (White, None), (Cover, B8), (Cyan, None), (Clrrb, B8), (Clrrb_pr, B8), (Cyan, None), (Cyan, None), (Rfi, B8), (Cyan, None), (Cyan, None), (Cyan, None), (Bsw_0, B8), (Bsw_1, B8), (Cyan, None), (Cyan, None),
                (Epc, B8), (Cyan, None), (Cyan, None), (Cyan, None), (Cyan, None), (Cyan, None), (Cyan, None), (Cyan, None), (Vmsw_0, B8), (Vmsw_1, B8), (Cyan, None), (Cyan, None), (Cyan, None), (Cyan, None), (Cyan, None), (Cyan, None),
                (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None),
                (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None), (Brown, None)
            ];

            let index = word[27..33].load::<u8>();
            if index == 0b10000 {
                // `Indirect Branch (Table 4-49)`
                const TABLE4_49: [(Opcode, OperandEncodingB); 8] = [
                    (Br_cond, B4), (Br_ia, B4), (Brown, None), (Brown, None),
                    (Brown, None), (Brown, None), (Brown, None), (Brown, None),
                ];
                let subindex = word[6..9].load::<u8>();
                TABLE4_49[subindex as usize]
            } else if index == 0b10001 {
                // `Indirect Return (Table 4-50)`
                const TABLE4_50: [(Opcode, OperandEncodingB); 8] = [
                    (Brown, None), (Brown, None), (Brown, None), (Brown, None),
                    (Br_ret, B4), (Brown, None), (Brown, None), (Brown, None),
                ];
                let subindex = word[6..9].load::<u8>();
                TABLE4_50[subindex as usize]
            } else {
                TABLE4_48[index as usize]
            }
        },
        0x1 => {
            (Br_call_bwh_ph_dh, B5)
        },
        0x2 => {
            // `Table 4-55 Indirect Predict/Nop/Hint Opcode Extensions`
            let x6 = word[27..33].load::<u8>();
            if x6 == 0b000000 {
                (Nop_b, B9)
            } else if x6 == 0b000001 {
                (Hint_b, B9)
            } else if x6 == 0b010000 {
                (Brp, B7)
            } else if x6 == 0b010001 {
                (Brp_ret, B7)
            } else {
                (White, None)
            }
        },
        0x3 => { (White, None) },
        0x4 => {
            // `Table 4-47 IP-Relative Branch Types`
            const TABLE4_47: [(Opcode, OperandEncodingB); 8] = [
                (Br_cond, B1), (Brown, None), (Br_wexit, B1), (Br_wtop, B1),
                (Brown, None), (Br_cloop, B2), (Br_cexit, B2), (Br_ctop, B2),
            ];
            let btype = word[6..9].load::<u8>();
            TABLE4_47[btype as usize]
        },
        0x5 => {
            (Br_call_bwh_ph_dh, B3)
        },
        0x6 => { (White, None) },
        0x7 => {
            (Brp_ipwh_ih, B6)
        },
        0x8 => { (Brown, None) },
        0x9 => { (Brown, None) },
        0xa => { (Brown, None) },
        0xb => { (Brown, None) },
        0xc => { (Brown, None) },
        0xd => { (Brown, None) },
        0xe => { (Brown, None) },
        0xf => { (Brown, None) },
        _ => { unreachable!() },
    }
}

fn get_f_opcode_and_encoding(tag: u8, word: &BitSlice<Lsb0, u8>) -> (Opcode, OperandEncodingF) {
    use Opcode::*;
    use OperandEncodingF::*;

    // `Table 4-63 Floating-point Status Field Completer` maps `sf` bits (35:34) to a FPSR status
    // field (.s0, .s1, .s2, .s3). the mapping is "bit value N" -> ".sN", so this table is
    // implicit.

    match tag {
        0x0 => {
            // `Table 4-59 Miscellaneous Floating-point 1-bit Opcode Extensions`
            if word[33] {
                if word[36] {
                    (Frcpa, F6)
                } else {
                    (Frsqta, F7)
                }
            } else {
                // `Table 4-60 Opcode 0 Miscellaneous Floating-point 6-bit Opcode Extensions`
                // `1-bit Ext (Table 4-68)` handled independently,
                const TABLE4_60: [(Opcode, OperandEncodingF); 64] = [
                    (Break_f, F15), (Purple, None), (Purple, None), (Purple, None),
                    (Fsetc, F12), (Fclrf, F13), (Purple, None), (Purple, None),
                    (Fchkf, F14), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Fmerge_s, F9), (Fmerge_ns, F9), (Fmerge_se, F9), (Purple, None),
                    (Fmin, F8), (Fmax, F8), (Famin, F8), (Famax, F8),
                    (Fcvt_fx, F10), (Fcvt_fxu, F10), (Fcvt_fx_trunc, F10), (Fcvt_fxu_trunc, F10),
                    (Fcvt_xf, F11), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Fpack, F9), (Purple, None), (Purple, None), (Purple, None),
                    (Fand, F9), (Fandcm, F9), (For, F9), (Fxor, F9),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Fswap, F9), (Fswap_nl, F9), (Fswap_nr, F9), (Purple, None),
                    (Purple, None), (Fmix_lr, F9), (Fmix_r, F9), (Fmix_l, F9),
                    (Fsxt_r, F9), (Fsxt_l, F9), (Purple, None), (Purple, None),
                ];
                let index = word[27..33].load::<u8>();
                if index == 0b00001 {
                    // `Table 4-68 `
                    if word[26] {
                        (Hint_f, F16)
                    } else {
                        (Nop_f, F16)
                    }
                } else {
                    TABLE4_60[index as usize]
                }
            }
        },
        0x1 => {
            // `Table 4-59 Miscellaneous Floating-point 1-bit Opcode Extensions`
            if word[33] {
                if word[36] {
                    (Fprcpa, F6)
                } else {
                    (Fprsqrta, F7)
                }
            } else {
                // `Table 4-61 Opcode 1 Miscellaneous Floating-point 6-bit Opcode Extensions`
                const TABLE4_61: [(Opcode, OperandEncodingF); 64] = [
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Fpmerge_s, F9), (Fpmerge_ns, F9), (Fpmerge_se, F9), (Purple, None),
                    (Fpmin, F8), (Fpmax, F8), (Fpamin, F8), (Fpamax, F8),
                    (Fpcvt_fx, F10), (Fpcvt_fxu, F10), (Fpcvt_fx_trunc, F10), (Fpcvt_fxu_trunc, F10),
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
                TABLE4_61[word[27..33].load::<u8>() as usize]
            }
        },
        2 => { (Purple, None) },
        3 => { (Purple, None) },
        0x4 => {
            let index =
                (word[12] as u8) << 2 +
                (word[33] as u8) << 1 +
                (word[36] as u8);
            const TABLE4_66: [(Opcode, OperandEncodingF); 8] = [
                (Fcmp_eq, F4), (Fcmp_lt, F4), (Fcmp_le, F4), (Fcmp_unord, F4),
                (Fcmp_eq_unc, F4), (Fcmp_lt_unc, F4), (Fcmp_le_unc, F4), (Fcmp_unord_unc, F4),
            ];
            TABLE4_66[index as usize]
        },
        0x5 => {
            // `Table 4-67 Floating-point Class 1-bit Opcode Extensions`
            if word[12] {
                (Fclass_m_unc, F5)
            } else {
                (Fclass_m, F5)
            }
        },
        6 => { (Purple, None) },
        7 => { (Purple, None) },
        0x8 => {
            // from section 4.6.1.1
            if word[36] {
                (Fma_s_sf, F1)
            } else {
                (Fma_sf, F1)
            }
        },
        0x9 => {
            // from section 4.6.1.1
            if word[36] {
                (Fpma_sf, F1)
            } else {
                (Fma_d_sf, F1)
            }
        },
        0xa => {
            // from section 4.6.1.1
            if word[36] {
                (Fms_s_sf, F1)
            } else {
                (Fms_sf, F1)
            }
        },
        0xb => {
            // from section 4.6.1.1
            if word[36] {
                (Fpms_sf, F1)
            } else {
                (Fms_d_sf, F1)
            }
        },
        0xc => {
            // from section 4.6.1.1
            if word[36] {
                (Fnma_s_sf, F1)
            } else {
                (Fnma_sf, F1)
            }
        },
        0xd => {
            // from section 4.6.1.1
            if word[36] {
                (Fpnma_sf, F1)
            } else {
                (Fnma_d_sf, F1)
            }
        },
        0xe => {
            // from section 4.6.1.2 and 4.6.2
            if word[36] {
                const TABLE_SECTION_4_6_1_2: [(Opcode, OperandEncodingF); 4] = [
                    (Xma_l, F2),
                    (White, None), // TODO: what exactly happens with x2 == 1? not mentioned?
                    (Xma_hu, F2),
                    (Xma_h, F2),
                ];
                TABLE_SECTION_4_6_1_2[word[34..36].load::<u8>() as usize]
            } else {
                (Fselect, F3)
            }
        },
        0xf => { (Purple, None) },
        _ => { unreachable!() },
    }
}

fn get_i_opcode_and_encoding(tag: u8, word: &BitSlice<Lsb0, u8>) -> (Opcode, OperandEncodingI) {
    use Opcode::*;
    use OperandEncodingI::*;

    match tag {
        0 => {
            let x3 = word[33..36].load::<u8>();
            // `Table 4-24 Misc I-Unit 3-bit Opcode Extensions`
            if x3 == 0 {
                // SIDEWAYS
                // `Table 4-25 Misc I-Unit 6-bit Opcode Extensions`
                const TABLE4_25: [(Opcode, OperandEncodingI); 64] = [
                    (Break_i, I19), (Zxt1, I29), (Purple, None), (Mov_from_ip, I25),
                    // `1-bit Ext (Table 4-26)` is handled independently
                    (Purple, None), (Zxt2, I29), (Purple, None), (Mov_from_b, I22),
                    (Purple, None), (Zxt4, I29), (Purple, None), (Mov_i_from_ar, I28),
                    (Purple, None), (Purple, None), (Purple, None), (Mov_from_pr, I25),
                    (Purple, None), (Sxt1, I29), (Purple, None), (Purple, None),
                    (Purple, None), (Sxt2, I29), (Purple, None), (Purple, None),
                    (Purple, None), (Sxt4, I29), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Czx1_l, I29), (Purple, None), (Purple, None),
                    (Purple, None), (Czx2_l, I29), (Purple, None), (Purple, None),
                    (Mov_i_to_ar_imm, I27), (Purple, None), (Mov_i_to_ar, I26), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Czx1_r, I29), (Purple, None), (Purple, None),
                    (Purple, None), (Czx2_r, I29), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                ];
                let index = word[27..33].load::<u8>();
                if index == 0b00001 {
                    // `1-bit Ext (Table 4-26)`
                    return if word[26] {
                        (Hint_i, I18)
                    } else {
                        (Nop_i, I18)
                    }
                }
                TABLE4_25[index as usize]
            } else {
                // `Table 4-24 Misc I-Unit 3-bit Opcode Extensions`
                const TABLE4_24: [(Opcode, OperandEncodingI); 8] = [
                    (Purple, None),
                    (Chk_s_i_int, I20),
                    (Mov_to_pr_rot_imm, I24),
                    (Mov_to_pr, I23),
                    (Purple, None),
                    (Purple, None),
                    (Purple, None),
                    (Mov_to_b, I21),
                ];
                TABLE4_24[x3 as usize]
            }
        },
        1 => { (Purple, None) },
        2 => { (Purple, None) },
        3 => { (Purple, None) },
        4 => { (Dep, I15) },
        5 => {
            let index = word[34..36].load::<u8>();

            // `Table 4-23 Test Bit Opcode Extensions`
            // this table is indexed by bits 40:37, 35:34, 33, 36, 12, 13, and 19, in that order.
            // bits 40:37, 35:34, are always zero, so the actual index is constructed from bits 33,
            // 36, 12, 13, and 19
            const TABLE4_23: [(Opcode, OperandEncodingI); 32] = [
                // bit 19 == 0
                (Tbit_z, I16), (Tnat_z, I17), (Tbit_z_unc, I16), (Tnat_z_unc, I17),
                (Tbit_z_and, I16), (Tnat_z_and, I17), (Tbit_nz_and, I16), (Tnat_nz_and, I17),
                (Tbit_z_or, I16), (Tnat_z_or, I17), (Tbit_nz_or, I16), (Tnat_nz_or, I17),
                (Tbit_z_or_andcm, I16), (Tnat_z_or_andcm, I17), (Tbit_nz_or_andcm, I16), (Tnat_nz_or_andcm, I17),
                // bit 19 == 1
                (Tbit_z, I16), (Tf_z, I30), (Tbit_z_unc, I16), (Tf_z_nc, I30),
                (Tbit_z_and, I16), (Tf_z_and, I30), (Tbit_nz_and, I16), (Tf_nz_and, I30),
                (Tbit_z_or, I16), (Tf_z_or, I30), (Tbit_nz_or, I16), (Tf_nz_or, I30),
                (Tbit_z_or_andcm, I16), (Tf_z_or_andcm, I30), (Tbit_nz_or_andcm, I16), (Tf_nz_or_andcm, I30),
            ];

            let table4_23_index =
                (word[19] as u8) << 4 +
                (word[33] as u8) << 3 +
                (word[36] as u8) << 2 +
                (word[12] as u8) << 1 +
                (word[13] as u8);

            if word[33] {
                if word[26] {
                    match index {
                        0 => TABLE4_23[table4_23_index as usize],
                        1 => (Dep_z_imm, I13),
                        2 => (Purple, None),
                        3 => (Dep_imm, I14),
                        _ => { unreachable!() },
                    }
                } else {
                    match index {
                        0 => TABLE4_23[table4_23_index as usize],
                        1 => (Dep_z, I12),
                        2 => (Purple, None),
                        3 => (Dep_imm, I14),
                        _ => { unreachable!() },
                    }
                }
            } else {
                if word[13] {
                    match index {
                        0 => TABLE4_23[table4_23_index as usize],
                        1 => (Extr, I11),
                        2 => (Purple, None),
                        3 => (Shrp, I10),
                        _ => { unreachable!() },
                    }
                } else {
                    match index {
                        0 => TABLE4_23[table4_23_index as usize],
                        1 => (Extr_u, I11),
                        2 => (Purple, None),
                        3 => (Shrp, I10),
                        _ => { unreachable!() },
                    }
                }
            }
        },
        6 => { (Purple, None) },
        7 => {
            // partial interpretation of table 4-16, `v_e == 1`
            if word[32] {
                (Purple, None)
            } else {
                // `Table 4-16 Multimedia and Variable Shift 1-bit Opcode Extensions`
                // (`v_e == 0`, since `v_e == 1` parts of this table are all undefined)
                const TABLE4_16: [&'static [(Opcode, OperandEncodingI); 64]; 4] = [
                    &TABLE4_17,
                    &TABLE4_18,
                    &TABLE4_19,
                    &TABLE4_20,
                ];

                // `Table 4-17 Multimedia Opcode 7 Size 1 2-bit Opcode Extensions`
                const TABLE4_17: [(Opcode, OperandEncodingI); 64] = [
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Pmin1_u, I2), (Purple, None), (Purple, None),
                    (Unpack1_h, I2), (Pmax1_u, I2), (Unpack1_l, I2), (Purple, None),
                    (Mix1_r, I2), (Purple, None), (Mix1_l, I2), (Psad1, I1),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Mux1, I3), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                ];

                // `Table 4-18 Multimedia Opcode 7 Size 2 2-bit Opcode Extensions`
                const TABLE4_18: [(Opcode, OperandEncodingI); 64] = [
                    (Pshr2_u_var, I5), (Pmpyshr2_u, I1), (Pshr2_var, I5), (Pmpyshr2, I1),
                    (Pshl1_var, I7), (Pmpyshr2_u, I1), (Purple, None), (Pmpyshr2, I1),
                    (Purple, None), (Pmpyshr2_u, I1), (Purple, None), (Pmpyshr2, I1),
                    (Purple, None), (Pmpyshr2_u, I1), (Purple, None), (Pmpyshr2, I1),
                    (Purple, None), (Pshr2_u_fixed, I6), (Purple, None), (Pshr2_fixed, I6),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Popcnt, I9), (Purple, None), (Purple, None),
                    (Purple, None), (Clz, I9), (Purple, None), (Purple, None),
                    (Pack2_uss, I2), (Purple, None), (Pack2_sss, I2), (Pmin2, I2),
                    (Unpack2_h, I2), (Purple, None), (Unpack2_l, I2), (Pmax2, I2),
                    (Mix2_r, I2), (Purple, None), (Mix2_l, I2), (Purple, None),
                    (Purple, None), (Pmpy2_r, I2), (Purple, None), (Pmpy2_l, I2),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Pshl2_fixed, I8), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Mux2, I4), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                ];

                // `Table 4-19 Multimedia Opcode 7 Size 4 2-bit Opcode Extensions`
                const TABLE4_19: [(Opcode, OperandEncodingI); 64] = [
                    (Pshr4_u_var, I5), (Purple, None), (Pshr4_var, I5), (Purple, None),
                    (Pshl4_var, I7), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Mpy4, I2), (Purple, None), (Mpyshl4, I2),
                    (Pshr4_u_fixed, I6), (Purple, None), (Pshr4_fixed, I6), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Pack4_sss, I2), (Purple, None),
                    (Unpack4_h, I2), (Purple, None), (Unpack4_l, I2), (Purple, None),
                    (Mix4_r, I2), (Purple, None), (Mix4_l, I2), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Pshl4_fixed, I8), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                    (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                ];

                const TABLE4_20: [(Opcode, OperandEncodingI); 64] = [
                    (Shr_u_var, I5), (Purple, None), (Shr_var, I5), (Purple, None),
                    (Shl_var, I7), (Purple, None), (Purple, None), (Purple, None),
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

                let index = (word[36] as u8) << 1 + (word[33] as u8);
                let inner_index = word[28..32].load::<u8>() + (word[34..35].load::<u8>() << 4);
                TABLE4_16[index as usize][inner_index as usize]
            }
        },
        _ => {
            unreachable!("m major op > 7 are a-type instructions");
        }
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
                const TABLE4_42: [(Opcode, OperandEncodingM); 8] = [
                    (Purple, None),
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
                const TABLE4_44: [(Opcode, OperandEncodingM); 8] = [
                    (Purple, None),
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
                Option::None
            ];

            // `Table 4-30 Integer Load/Store Opcode Extensions`
            // NOTE: this differs from the manual, which i believe to be in error. the manual's
            // `Table 4-30` lists operand encodings of `M2` in all places that are `M1` here, and is
            // functionally a duplicate of `Table 4-31` as a result. looking at the `Int Load`
            // encoding specifically, it indicates that bit 36 (named `m` here) being 0 should pick
            // `M1` encodings. the manual also lists `M2` twice, so it seems likely at least one
            // `M2` is in error. the first `M2` matches with the `M1` encoding specified in `Table
            // 4-4 Instruction Format Summary`, so that's likely the one in error.
            // `M6` from `Table 4-30` appears to also be in error, and should name `M4`
            const TABLE4_30: [(Opcode, OperandEncodingM); 64] = [
                (Ld1, M1), (Ld2, M1), (Ld4, M1), (Ld8, M1),
                (Ld1_s, M1), (Ld2_s, M1), (Ld4_s, M1), (Ld8_s, M1),
                (Ld1_a, M1), (Ld2_a, M1), (Ld4_a, M1), (Ld8_a, M1),
                (Ld1_sa, M1), (Ld2_sa, M1), (Ld4_sa, M1), (Ld8_sa, M1),
                (Ld1_bias, M1), (Ld2_bias, M1), (Ld4_bias, M1), (Ld8_bias, M1),
                (Ld1_acq, M1), (Ld2_acq, M1), (Ld4_acq, M1), (Ld8_acq, M1),
                (Purple, None), (Purple, None), (Purple, None), (Ld8_fill, M1),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Ld1_c_clr, M1), (Ld2_c_clr, M1), (Ld4_c_clr, M1), (Ld8_c_clr, M1),
                (Ld1_c_nc, M1), (Ld2_c_nc, M1), (Ld4_c_nc, M1), (Ld8_c_nc, M1),
                (Ld1_c_clr_acq, M1), (Ld2_c_clr_acq, M1), (Ld4_c_clr_acq, M1), (Ld8_c_clr_acq, M1),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (St1, M4), (St2, M4), (St4, M4), (St8, M4),
                (St1_rel, M4), (St2_rel, M4), (St4_rel, M4), (St8_rel, M4),
                (Purple, None), (Purple, None), (Purple, None), (St8_spill, M4),
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
            // `M6` from `Table 4-33` appears to also be in error, and should name `M4`
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
                (St16, M4), (Purple, None), (Purple, None), (Purple, None),
                (St16_rel, M4), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
            ];

            let index = ((word[36] as u8) << 1) + (word[27] as u8);
            eprintln!("table {}", index);
            eprintln!("  subidx {:b}", word[30..36].load::<u8>() as usize);
            eprintln!("  x6l {:#b}, x6h {:#b}", word[30..32].load::<u8>(), word[32..36].load::<u8>());
            if let Some(op_table) = TABLE4_28[index as usize] {
                op_table[word[30..36].load::<u8>() as usize]
            } else {
                (Purple, None)
            }
        },
        5 => {
            // `Table 4-32 Integer Load/Store +Imm Opcode Extensions`
            const TABLE4_32: [(Opcode, OperandEncodingM); 64] = [
                (Ld1, M3), (Ld2, M3), (Ld4, M3), (Ld8, M3),
                (Ld1_s, M3), (Ld2_s, M3), (Ld4_s, M3), (Ld8_s, M3),
                (Ld1_a, M3), (Ld2_a, M3), (Ld4_a, M3), (Ld8_a, M3),
                (Ld1_sa, M3), (Ld2_sa, M3), (Ld4_sa, M3), (Ld8_sa, M3),
                (Ld1_bias, M3), (Purple, M3), (Purple, M3), (Ld8_bias, M3),
                (Ld1_acq, M3), (Purple, M3), (Purple, M3), (Ld8_acq, M3),
                (Purple, None), (Purple, None), (Purple, None), (Ld8_fill, M3),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Ld1_c_clr, M3), (Ld2_c_clr, M3), (Ld4_c_clr, M3), (Ld8_c_clr, M3),
                (Ld1_c_nc, M3), (Ld2_c_nc, M3), (Ld4_c_nc, M3), (Ld8_c_nc, M3),
                (Ld1_c_clr_acq, M3), (Ld2_c_clr_acq, M3), (Ld4_c_clr_acq, M3), (Ld8_c_clr_acq, M3),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (St1, M5), (St2, M5), (St4, M5), (St8, M5),
                (St1_rel, M5), (St2_rel, M5), (St4_rel, M5), (St8_rel, M5),
                (Purple, None), (Purple, None), (Purple, None), (St8_spill, M5),
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
            // NOTE: manual is in error. `M9` in this table ought to be `M6`.
            // NOTE: manual is in error. `M13` in this table ought to be `M9`.
            // NOTE: manual is in error. `M18` in this table ought to be `M13`.
            const TABLE4_34: [(Opcode, OperandEncodingM); 64] = [
                (Ldfe, M6), (Ldf8, M6), (Ldfs, M6), (Ldfd, M6),
                (Ldfe_s, M6), (Ldf8_s, M6), (Ldfs_s, M6), (Ldfd_s, M6),
                (Ldfe_a, M6), (Ldf8_a, M6), (Ldfp_a, M6), (Ldfd_a, M6),
                (Ldfe_sa, M6), (Ldf8_sa, M6), (Ldfp_sa, M6), (Ldfd_sa, M6),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Ldf_fill, M6),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Ldfe_c_clr, M6), (Ldf8_c_clr, M6), (Ldfs_c_clr, M6), (Ldfd_c_clr, M6),
                (Ldfe_c_nc, M6), (Ldf8_c_nc, M6), (Ldfs_c_nc, M6), (Ldfd_c_nc, M6),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Lfetch, M13), (Lfetch_excl, M13), (Lfetch_fault, M13), (Lfetch_fault_excl, M13),
                (Stfe, M9), (Stf8, M9), (Stfs, M9), (Stfd, M9),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Stf_spill, M9),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
            ];

            // `Table 4-35 Floating-point Load/Lfetch +Reg Opcode Extensions`
            // NOTE: manual is in error. `M20` in this table ought to be `M14`.
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
                (Lfetch, M14), (Lfetch_excl, M14), (Lfetch_fault, M14), (Lfetch_fault_excl, M14),
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
            // NOTE: manual is in error. `M22` in this table ought to be `M15`.
            const TABLE4_36: [(Opcode, OperandEncodingM); 64] = [
                (Ldfe, M8), (Ldf8, M8), (Ldfs, M8), (Ldfd, M8),
                (Ldfe_s, M8), (Ldf8_s, M8), (Ldfs_s, M8), (Ldfd_s, M8),
                (Ldfe_a, M8), (Ldf8_a, M8), (Ldfs_a, M8), (Ldfd_a, M8),
                (Ldfe_sa, M8), (Ldf8_sa, M8), (Ldfs_sa, M8), (Ldfd_sa, M8),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Ldf_fill, M8),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Ldfe_c_clr, M8), (Ldf8_c_clr, M8), (Ldfs_c_clr, M8), (Ldfd_c_clr, M8),
                (Ldfe_c_nc, M8), (Ldf8_c_nc, M8), (Ldfs_c_nc, M8), (Ldfd_c_nc, M8),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Lfetch, M15), (Lfetch_excl, M15), (Lfetch_fault, M15), (Lfetch_fault_excl, M15),
                (Stfe, M10), (Stf8, M10), (Stfs, M10), (Stfd, M10),
                (Purple, None), (Purple, None), (Purple, None), (Purple, None),
                (Purple, None), (Purple, None), (Purple, None), (Stf_spill, M10),
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
                        Option::None
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
                        (Purple, None), (Purple, None), (Pavg2, A9), (Pavg2_raz, A9),
                        (Purple, None), (Purple, None), (Pavgsub2, A9), (Purple, None),
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
                _ => {
                    unreachable!()
                }
            }
        }
        9 => (Addl, A5),
        0xa => (Purple, None),
        0xb => (Purple, None),
        0xc => {
            let x2 = word[34..36].load::<u8>();
            let encoding = if x2 > 1 {
                // `Table 4-11 Integer Compare Immediate Opcode Extensions`
                A8
            } else {
                // `Table 4-10 Integer Compare Opcode Extensions`
                if word[36] { A7 } else { A6 }
            };
            unimplemented!("type a, tag c");
        }
        0xd => {
            let x2 = word[34..36].load::<u8>();
            let encoding = if x2 > 1 {
                // `Table 4-11 Integer Compare Immediate Opcode Extensions`
                A8
            } else {
                // `Table 4-10 Integer Compare Opcode Extensions`
                if word[36] { A7 } else { A6 }
            };
            unimplemented!("type a, tag d");
        }
        0xe => {
            let x2 = word[34..36].load::<u8>();
            let encoding = if x2 > 1 {
                // `Table 4-11 Integer Compare Immediate Opcode Extensions`
                A8
            } else {
                // `Table 4-10 Integer Compare Opcode Extensions`
                if word[36] { A7 } else { A6 }
            };
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
    None,
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
    None,
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
    None,
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
    None,
    X1,
    X2,
    X3,
    X4,
    X5,
}

