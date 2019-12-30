/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of self software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and self permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use super::super::*;
use super::enums::*;
use super::instruction_fmt::*;
use super::op_code_fmt::*;
use super::op_kind_tables::*;
use std::{fmt, mem};

// GENERATOR-BEGIN: Flags
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct Flags;
impl Flags {
	pub(crate) const NONE: u32 = 0x0000_0000;
	pub(crate) const MODE16: u32 = 0x0000_0001;
	pub(crate) const MODE32: u32 = 0x0000_0002;
	pub(crate) const MODE64: u32 = 0x0000_0004;
	pub(crate) const FWAIT: u32 = 0x0000_0008;
	pub(crate) const LIG: u32 = 0x0000_0010;
	pub(crate) const WIG: u32 = 0x0000_0020;
	pub(crate) const WIG32: u32 = 0x0000_0040;
	pub(crate) const W: u32 = 0x0000_0080;
	pub(crate) const BROADCAST: u32 = 0x0000_0100;
	pub(crate) const ROUNDING_CONTROL: u32 = 0x0000_0200;
	pub(crate) const SUPPRESS_ALL_EXCEPTIONS: u32 = 0x0000_0400;
	pub(crate) const OP_MASK_REGISTER: u32 = 0x0000_0800;
	pub(crate) const ZEROING_MASKING: u32 = 0x0000_1000;
	pub(crate) const LOCK_PREFIX: u32 = 0x0000_2000;
	pub(crate) const XACQUIRE_PREFIX: u32 = 0x0000_4000;
	pub(crate) const XRELEASE_PREFIX: u32 = 0x0000_8000;
	pub(crate) const REP_PREFIX: u32 = 0x0001_0000;
	pub(crate) const REPNE_PREFIX: u32 = 0x0002_0000;
	pub(crate) const BND_PREFIX: u32 = 0x0004_0000;
	pub(crate) const HINT_TAKEN_PREFIX: u32 = 0x0008_0000;
	pub(crate) const NOTRACK_PREFIX: u32 = 0x0010_0000;
	pub(crate) const NO_INSTRUCTION: u32 = 0x0020_0000;
	pub(crate) const NON_ZERO_OP_MASK_REGISTER: u32 = 0x0040_0000;
}
// GENERATOR-END: Flags

/// Opcode info, returned by [`Code::op_code()`] and [`Instruction::op_code()`]
///
/// [`Code::op_code()`]: enum.Code.html#method.op_code
/// [`Instruction::op_code()`]: struct.Instruction.html#method.op_code
#[derive(Debug)]
pub struct OpCodeInfo {
	op_code_string: String,
	instruction_string: String,
	flags: u32, // Flags
	code: Code,
	op_code: u16,
	encoding: EncodingKind,
	operand_size: u8,
	address_size: u8,
	l: u8,
	tuple_type: TupleType,
	table: OpCodeTableKind,
	mandatory_prefix: MandatoryPrefix,
	group_index: i8,
	op0_kind: OpCodeOperandKind,
	op1_kind: OpCodeOperandKind,
	op2_kind: OpCodeOperandKind,
	op3_kind: OpCodeOperandKind,
	op4_kind: OpCodeOperandKind,
	lkind: LKind,
}

impl OpCodeInfo {
	pub(crate) fn new(code: Code, dword1: u32, dword2: u32, dword3: u32, sb: &mut String) -> Self {
		let mut flags = Flags::NONE;
		if code == Code::INVALID || code >= Code::DeclareByte {
			flags |= Flags::NO_INSTRUCTION;
		}
		let op_code = (dword1 >> EncFlags1::OP_CODE_SHIFT) as u16;

		let op0_kind;
		let op1_kind;
		let op2_kind;
		let op3_kind;
		let op4_kind;
		let l;
		let mandatory_prefix;
		let table;
		let group_index;
		let tuple_type;
		let operand_size;
		let address_size;
		let lkind;

		let encoding = unsafe { mem::transmute(((dword1 >> EncFlags1::ENCODING_SHIFT) & EncFlags1::ENCODING_MASK) as u8) };
		match encoding {
			EncodingKind::Legacy => {
				op0_kind = LEGACY_OP_KINDS[((dword3 >> LegacyFlags3::OP0_SHIFT) & LegacyFlags3::OP_MASK) as usize];
				op1_kind = LEGACY_OP_KINDS[((dword3 >> LegacyFlags3::OP1_SHIFT) & LegacyFlags3::OP_MASK) as usize];
				op2_kind = LEGACY_OP_KINDS[((dword3 >> LegacyFlags3::OP2_SHIFT) & LegacyFlags3::OP_MASK) as usize];
				op3_kind = LEGACY_OP_KINDS[((dword3 >> LegacyFlags3::OP3_SHIFT) & LegacyFlags3::OP_MASK) as usize];
				op4_kind = OpCodeOperandKind::None;

				mandatory_prefix = match unsafe {
					mem::transmute::<u8, MandatoryPrefixByte>(
						((dword2 >> LegacyFlags::MANDATORY_PREFIX_BYTE_SHIFT) & LegacyFlags::MANDATORY_PREFIX_BYTE_MASK) as u8,
					)
				} {
					MandatoryPrefixByte::None => {
						if (dword2 & LegacyFlags::HAS_MANDATORY_PREFIX) != 0 {
							MandatoryPrefix::PNP
						} else {
							MandatoryPrefix::None
						}
					}
					MandatoryPrefixByte::P66 => MandatoryPrefix::P66,
					MandatoryPrefixByte::PF3 => MandatoryPrefix::PF3,
					MandatoryPrefixByte::PF2 => MandatoryPrefix::PF2,
				};

				table = match unsafe {
					mem::transmute::<u8, LegacyOpCodeTable>(
						((dword2 >> LegacyFlags::LEGACY_OP_CODE_TABLE_SHIFT) & LegacyFlags::LEGACY_OP_CODE_TABLE_MASK) as u8,
					)
				} {
					LegacyOpCodeTable::Normal => OpCodeTableKind::Normal,
					LegacyOpCodeTable::Table0F => OpCodeTableKind::T0F,
					LegacyOpCodeTable::Table0F38 => OpCodeTableKind::T0F38,
					LegacyOpCodeTable::Table0F3A => OpCodeTableKind::T0F3A,
				};

				group_index = if (dword2 & LegacyFlags::HAS_GROUP_INDEX) == 0 { -1 } else { ((dword2 >> LegacyFlags::GROUP_SHIFT) & 7) as i8 };
				tuple_type = TupleType::None;

				if (flags & Flags::NO_INSTRUCTION) != 0 {
					flags |= Flags::MODE16 | Flags::MODE32 | Flags::MODE64;
				} else {
					flags |= match unsafe {
						mem::transmute::<u8, Encodable>(((dword2 >> LegacyFlags::ENCODABLE_SHIFT) & LegacyFlags::ENCODABLE_MASK) as u8)
					} {
						Encodable::Any => Flags::MODE16 | Flags::MODE32 | Flags::MODE64,
						Encodable::Only1632 => Flags::MODE16 | Flags::MODE32,
						Encodable::Only64 => Flags::MODE64,
					};
				}

				flags |= match unsafe {
					mem::transmute::<u8, AllowedPrefixes>(
						((dword2 >> LegacyFlags::ALLOWED_PREFIXES_SHIFT) & LegacyFlags::ALLOWED_PREFIXES_MASK) as u8,
					)
				} {
					// GENERATOR-BEGIN: AllowedPrefixes
					// ⚠️This was generated by GENERATOR!🦹‍♂️
					AllowedPrefixes::None => Flags::NONE,
					AllowedPrefixes::Bnd => Flags::BND_PREFIX,
					AllowedPrefixes::BndNotrack => Flags::BND_PREFIX | Flags::NOTRACK_PREFIX,
					AllowedPrefixes::HintTakenBnd => Flags::BND_PREFIX | Flags::HINT_TAKEN_PREFIX,
					AllowedPrefixes::Lock => Flags::LOCK_PREFIX,
					AllowedPrefixes::Rep => Flags::REP_PREFIX,
					AllowedPrefixes::RepRepne => Flags::REP_PREFIX | Flags::REPNE_PREFIX,
					AllowedPrefixes::XacquireXreleaseLock => Flags::LOCK_PREFIX | Flags::XACQUIRE_PREFIX | Flags::XRELEASE_PREFIX,
					AllowedPrefixes::Xrelease => Flags::XRELEASE_PREFIX,
					// GENERATOR-END: AllowedPrefixes
				};
				if (dword2 & LegacyFlags::FWAIT) != 0 {
					flags |= Flags::FWAIT;
				}

				operand_size = match unsafe {
					mem::transmute::<u8, OperandSize>(((dword2 >> LegacyFlags::OPERAND_SIZE_SHIFT) & LegacyFlags::OPERAND_SIZE_MASK) as u8)
				} {
					OperandSize::None => 0,
					OperandSize::Size16 => 16,
					OperandSize::Size32 => 32,
					OperandSize::Size64 => 64,
				};

				address_size = match unsafe {
					mem::transmute::<u8, AddressSize>(((dword2 >> LegacyFlags::ADDRESS_SIZE_SHIFT) & LegacyFlags::ADDRESS_SIZE_MASK) as u8)
				} {
					AddressSize::None => 0,
					AddressSize::Size16 => 16,
					AddressSize::Size32 => 32,
					AddressSize::Size64 => 64,
				};

				l = 0;
				lkind = LKind::None;
			}

			EncodingKind::VEX => {
				op0_kind = VEX_OP_KINDS[((dword3 >> VexFlags3::OP0_SHIFT) & VexFlags3::OP_MASK) as usize];
				op1_kind = VEX_OP_KINDS[((dword3 >> VexFlags3::OP1_SHIFT) & VexFlags3::OP_MASK) as usize];
				op2_kind = VEX_OP_KINDS[((dword3 >> VexFlags3::OP2_SHIFT) & VexFlags3::OP_MASK) as usize];
				op3_kind = VEX_OP_KINDS[((dword3 >> VexFlags3::OP3_SHIFT) & VexFlags3::OP_MASK) as usize];
				op4_kind = VEX_OP_KINDS[((dword3 >> VexFlags3::OP4_SHIFT) & VexFlags3::OP_MASK) as usize];

				mandatory_prefix = match unsafe {
					mem::transmute::<u8, MandatoryPrefixByte>(
						((dword2 >> VexFlags::MANDATORY_PREFIX_BYTE_SHIFT) & VexFlags::MANDATORY_PREFIX_BYTE_MASK) as u8,
					)
				} {
					MandatoryPrefixByte::None => MandatoryPrefix::PNP,
					MandatoryPrefixByte::P66 => MandatoryPrefix::P66,
					MandatoryPrefixByte::PF3 => MandatoryPrefix::PF3,
					MandatoryPrefixByte::PF2 => MandatoryPrefix::PF2,
				};

				table = match unsafe {
					mem::transmute::<u8, VexOpCodeTable>(((dword2 >> VexFlags::VEX_OP_CODE_TABLE_SHIFT) & VexFlags::VEX_OP_CODE_TABLE_MASK) as u8)
				} {
					VexOpCodeTable::Table0F => OpCodeTableKind::T0F,
					VexOpCodeTable::Table0F38 => OpCodeTableKind::T0F38,
					VexOpCodeTable::Table0F3A => OpCodeTableKind::T0F3A,
				};

				group_index = if (dword2 & VexFlags::HAS_GROUP_INDEX) == 0 { -1 } else { ((dword2 >> VexFlags::GROUP_SHIFT) & 7) as i8 };
				tuple_type = TupleType::None;

				flags |= match unsafe { mem::transmute::<u8, Encodable>(((dword2 >> VexFlags::ENCODABLE_SHIFT) & VexFlags::ENCODABLE_MASK) as u8) } {
					Encodable::Any => Flags::MODE16 | Flags::MODE32 | Flags::MODE64,
					Encodable::Only1632 => Flags::MODE16 | Flags::MODE32,
					Encodable::Only64 => Flags::MODE64,
				};
				operand_size = 0;
				address_size = 0;
				match unsafe {
					mem::transmute::<u8, VexVectorLength>(((dword2 >> VexFlags::VEX_VECTOR_LENGTH_SHIFT) & VexFlags::VEX_VECTOR_LENGTH_MASK) as u8)
				} {
					VexVectorLength::LZ => {
						lkind = LKind::LZ;
						l = 0;
					}
					VexVectorLength::L0 => {
						lkind = LKind::L0;
						l = 0;
					}
					VexVectorLength::L1 => {
						lkind = LKind::L0;
						l = 1;
					}
					VexVectorLength::L128 => {
						lkind = LKind::L128;
						l = 0;
					}
					VexVectorLength::L256 => {
						lkind = LKind::L128;
						l = 1;
					}
					VexVectorLength::LIG => {
						lkind = LKind::None;
						l = 0;
						flags |= Flags::LIG;
					}
				}

				match unsafe { mem::transmute::<u8, WBit>(((dword2 >> VexFlags::WBIT_SHIFT) & VexFlags::WBIT_MASK) as u8) } {
					WBit::W0 => {}
					WBit::W1 => flags |= Flags::W,
					WBit::WIG => flags |= Flags::WIG,
					WBit::WIG32 => flags |= Flags::WIG32,
				}
			}

			EncodingKind::EVEX => {
				op0_kind = EVEX_OP_KINDS[((dword3 >> EvexFlags3::OP0_SHIFT) & EvexFlags3::OP_MASK) as usize];
				op1_kind = EVEX_OP_KINDS[((dword3 >> EvexFlags3::OP1_SHIFT) & EvexFlags3::OP_MASK) as usize];
				op2_kind = EVEX_OP_KINDS[((dword3 >> EvexFlags3::OP2_SHIFT) & EvexFlags3::OP_MASK) as usize];
				op3_kind = EVEX_OP_KINDS[((dword3 >> EvexFlags3::OP3_SHIFT) & EvexFlags3::OP_MASK) as usize];
				op4_kind = OpCodeOperandKind::None;

				mandatory_prefix = match unsafe {
					mem::transmute::<u8, MandatoryPrefixByte>(
						((dword2 >> EvexFlags::MANDATORY_PREFIX_BYTE_SHIFT) & EvexFlags::MANDATORY_PREFIX_BYTE_MASK) as u8,
					)
				} {
					MandatoryPrefixByte::None => MandatoryPrefix::PNP,
					MandatoryPrefixByte::P66 => MandatoryPrefix::P66,
					MandatoryPrefixByte::PF3 => MandatoryPrefix::PF3,
					MandatoryPrefixByte::PF2 => MandatoryPrefix::PF2,
				};

				table = match unsafe {
					mem::transmute::<u8, EvexOpCodeTable>(
						((dword2 >> EvexFlags::EVEX_OP_CODE_TABLE_SHIFT) & EvexFlags::EVEX_OP_CODE_TABLE_MASK) as u8,
					)
				} {
					EvexOpCodeTable::Table0F => OpCodeTableKind::T0F,
					EvexOpCodeTable::Table0F38 => OpCodeTableKind::T0F38,
					EvexOpCodeTable::Table0F3A => OpCodeTableKind::T0F3A,
				};

				group_index = if (dword2 & EvexFlags::HAS_GROUP_INDEX) == 0 { -1 } else { ((dword2 >> EvexFlags::GROUP_SHIFT) & 7) as i8 };
				tuple_type = unsafe { mem::transmute::<u8, TupleType>(((dword2 >> EvexFlags::TUPLE_TYPE_SHIFT) & EvexFlags::TUPLE_TYPE_MASK) as u8) };

				flags |= match unsafe { mem::transmute::<u8, Encodable>(((dword2 >> EvexFlags::ENCODABLE_SHIFT) & EvexFlags::ENCODABLE_MASK) as u8) }
				{
					Encodable::Any => Flags::MODE16 | Flags::MODE32 | Flags::MODE64,
					Encodable::Only1632 => Flags::MODE16 | Flags::MODE32,
					Encodable::Only64 => Flags::MODE64,
				};
				operand_size = 0;
				address_size = 0;
				l = ((dword2 >> EvexFlags::EVEX_VECTOR_LENGTH_SHIFT) & EvexFlags::EVEX_VECTOR_LENGTH_MASK) as u8;

				match unsafe { mem::transmute::<u8, WBit>(((dword2 >> EvexFlags::WBIT_SHIFT) & EvexFlags::WBIT_MASK) as u8) } {
					WBit::W0 => {}
					WBit::W1 => flags |= Flags::W,
					WBit::WIG => flags |= Flags::WIG,
					WBit::WIG32 => flags |= Flags::WIG32,
				}
				if (dword2 & EvexFlags::LIG) != 0 {
					flags |= Flags::LIG;
				}
				if (dword2 & EvexFlags::B) != 0 {
					flags |= Flags::BROADCAST;
				}
				if (dword2 & EvexFlags::ER) != 0 {
					flags |= Flags::ROUNDING_CONTROL;
				}
				if (dword2 & EvexFlags::SAE) != 0 {
					flags |= Flags::SUPPRESS_ALL_EXCEPTIONS;
				}
				if (dword2 & EvexFlags::K1) != 0 {
					flags |= Flags::OP_MASK_REGISTER;
				}
				if (dword2 & EvexFlags::Z) != 0 {
					flags |= Flags::ZEROING_MASKING;
				}
				lkind = LKind::L128;
				match code {
					// GENERATOR-BEGIN: NonZeroOpMaskRegister
					// ⚠️This was generated by GENERATOR!🦹‍♂️
					Code::EVEX_Vpgatherdd_xmm_k1_vm32x
					| Code::EVEX_Vpgatherdd_ymm_k1_vm32y
					| Code::EVEX_Vpgatherdd_zmm_k1_vm32z
					| Code::EVEX_Vpgatherdq_xmm_k1_vm32x
					| Code::EVEX_Vpgatherdq_ymm_k1_vm32x
					| Code::EVEX_Vpgatherdq_zmm_k1_vm32y
					| Code::EVEX_Vpgatherqd_xmm_k1_vm64x
					| Code::EVEX_Vpgatherqd_xmm_k1_vm64y
					| Code::EVEX_Vpgatherqd_ymm_k1_vm64z
					| Code::EVEX_Vpgatherqq_xmm_k1_vm64x
					| Code::EVEX_Vpgatherqq_ymm_k1_vm64y
					| Code::EVEX_Vpgatherqq_zmm_k1_vm64z
					| Code::EVEX_Vgatherdps_xmm_k1_vm32x
					| Code::EVEX_Vgatherdps_ymm_k1_vm32y
					| Code::EVEX_Vgatherdps_zmm_k1_vm32z
					| Code::EVEX_Vgatherdpd_xmm_k1_vm32x
					| Code::EVEX_Vgatherdpd_ymm_k1_vm32x
					| Code::EVEX_Vgatherdpd_zmm_k1_vm32y
					| Code::EVEX_Vgatherqps_xmm_k1_vm64x
					| Code::EVEX_Vgatherqps_xmm_k1_vm64y
					| Code::EVEX_Vgatherqps_ymm_k1_vm64z
					| Code::EVEX_Vgatherqpd_xmm_k1_vm64x
					| Code::EVEX_Vgatherqpd_ymm_k1_vm64y
					| Code::EVEX_Vgatherqpd_zmm_k1_vm64z
					| Code::EVEX_Vpscatterdd_vm32x_k1_xmm
					| Code::EVEX_Vpscatterdd_vm32y_k1_ymm
					| Code::EVEX_Vpscatterdd_vm32z_k1_zmm
					| Code::EVEX_Vpscatterdq_vm32x_k1_xmm
					| Code::EVEX_Vpscatterdq_vm32x_k1_ymm
					| Code::EVEX_Vpscatterdq_vm32y_k1_zmm
					| Code::EVEX_Vpscatterqd_vm64x_k1_xmm
					| Code::EVEX_Vpscatterqd_vm64y_k1_xmm
					| Code::EVEX_Vpscatterqd_vm64z_k1_ymm
					| Code::EVEX_Vpscatterqq_vm64x_k1_xmm
					| Code::EVEX_Vpscatterqq_vm64y_k1_ymm
					| Code::EVEX_Vpscatterqq_vm64z_k1_zmm
					| Code::EVEX_Vscatterdps_vm32x_k1_xmm
					| Code::EVEX_Vscatterdps_vm32y_k1_ymm
					| Code::EVEX_Vscatterdps_vm32z_k1_zmm
					| Code::EVEX_Vscatterdpd_vm32x_k1_xmm
					| Code::EVEX_Vscatterdpd_vm32x_k1_ymm
					| Code::EVEX_Vscatterdpd_vm32y_k1_zmm
					| Code::EVEX_Vscatterqps_vm64x_k1_xmm
					| Code::EVEX_Vscatterqps_vm64y_k1_xmm
					| Code::EVEX_Vscatterqps_vm64z_k1_ymm
					| Code::EVEX_Vscatterqpd_vm64x_k1_xmm
					| Code::EVEX_Vscatterqpd_vm64y_k1_ymm
					| Code::EVEX_Vscatterqpd_vm64z_k1_zmm
					| Code::EVEX_Vgatherpf0dps_vm32z_k1
					| Code::EVEX_Vgatherpf0dpd_vm32y_k1
					| Code::EVEX_Vgatherpf1dps_vm32z_k1
					| Code::EVEX_Vgatherpf1dpd_vm32y_k1
					| Code::EVEX_Vscatterpf0dps_vm32z_k1
					| Code::EVEX_Vscatterpf0dpd_vm32y_k1
					| Code::EVEX_Vscatterpf1dps_vm32z_k1
					| Code::EVEX_Vscatterpf1dpd_vm32y_k1
					| Code::EVEX_Vgatherpf0qps_vm64z_k1
					| Code::EVEX_Vgatherpf0qpd_vm64z_k1
					| Code::EVEX_Vgatherpf1qps_vm64z_k1
					| Code::EVEX_Vgatherpf1qpd_vm64z_k1
					| Code::EVEX_Vscatterpf0qps_vm64z_k1
					| Code::EVEX_Vscatterpf0qpd_vm64z_k1
					| Code::EVEX_Vscatterpf1qps_vm64z_k1
					| Code::EVEX_Vscatterpf1qpd_vm64z_k1
					// GENERATOR-END: NonZeroOpMaskRegister
					=> flags |= Flags::NON_ZERO_OP_MASK_REGISTER,
					_ => {}
				}
			}

			EncodingKind::XOP => {
				op0_kind = XOP_OP_KINDS[((dword3 >> XopFlags3::OP0_SHIFT) & XopFlags3::OP_MASK) as usize];
				op1_kind = XOP_OP_KINDS[((dword3 >> XopFlags3::OP1_SHIFT) & XopFlags3::OP_MASK) as usize];
				op2_kind = XOP_OP_KINDS[((dword3 >> XopFlags3::OP2_SHIFT) & XopFlags3::OP_MASK) as usize];
				op3_kind = XOP_OP_KINDS[((dword3 >> XopFlags3::OP3_SHIFT) & XopFlags3::OP_MASK) as usize];
				op4_kind = OpCodeOperandKind::None;

				mandatory_prefix = match unsafe {
					mem::transmute::<u8, MandatoryPrefixByte>(
						((dword2 >> XopFlags::MANDATORY_PREFIX_BYTE_SHIFT) & XopFlags::MANDATORY_PREFIX_BYTE_MASK) as u8,
					)
				} {
					MandatoryPrefixByte::None => MandatoryPrefix::PNP,
					MandatoryPrefixByte::P66 => MandatoryPrefix::P66,
					MandatoryPrefixByte::PF3 => MandatoryPrefix::PF3,
					MandatoryPrefixByte::PF2 => MandatoryPrefix::PF2,
				};

				table = match unsafe {
					mem::transmute::<u8, XopOpCodeTable>(((dword2 >> XopFlags::XOP_OP_CODE_TABLE_SHIFT) & XopFlags::XOP_OP_CODE_TABLE_MASK) as u8)
				} {
					XopOpCodeTable::XOP8 => OpCodeTableKind::XOP8,
					XopOpCodeTable::XOP9 => OpCodeTableKind::XOP9,
					XopOpCodeTable::XOPA => OpCodeTableKind::XOPA,
				};

				group_index = if (dword2 & XopFlags::HAS_GROUP_INDEX) == 0 { -1 } else { ((dword2 >> XopFlags::GROUP_SHIFT) & 7) as i8 };
				tuple_type = TupleType::None;

				flags |= match unsafe { mem::transmute::<u8, Encodable>(((dword2 >> XopFlags::ENCODABLE_SHIFT) & XopFlags::ENCODABLE_MASK) as u8) } {
					Encodable::Any => Flags::MODE16 | Flags::MODE32 | Flags::MODE64,
					Encodable::Only1632 => Flags::MODE16 | Flags::MODE32,
					Encodable::Only64 => Flags::MODE64,
				};
				operand_size = 0;
				address_size = 0;

				match unsafe { mem::transmute::<u8, WBit>(((dword2 >> XopFlags::WBIT_SHIFT) & XopFlags::WBIT_MASK) as u8) } {
					WBit::W0 => {}
					WBit::W1 => flags |= Flags::W,
					WBit::WIG => flags |= Flags::WIG,
					WBit::WIG32 => flags |= Flags::WIG32,
				}
				match unsafe {
					mem::transmute::<u8, XopVectorLength>(((dword2 >> XopFlags::XOP_VECTOR_LENGTH_SHIFT) & XopFlags::XOP_VECTOR_LENGTH_MASK) as u8)
				} {
					XopVectorLength::L128 => {
						l = 0;
						lkind = LKind::L128;
					}
					XopVectorLength::L256 => {
						l = 1;
						lkind = LKind::L128;
					}
					XopVectorLength::L0 => {
						l = 0;
						lkind = LKind::L0;
					}
					XopVectorLength::L1 => {
						l = 1;
						lkind = LKind::L0;
					}
				}
			}

			EncodingKind::D3NOW => {
				op0_kind = OpCodeOperandKind::mm_reg;
				op1_kind = OpCodeOperandKind::mm_or_mem;
				op2_kind = OpCodeOperandKind::None;
				op3_kind = OpCodeOperandKind::None;
				op4_kind = OpCodeOperandKind::None;
				mandatory_prefix = MandatoryPrefix::None;
				table = OpCodeTableKind::T0F;
				group_index = -1;
				tuple_type = TupleType::None;

				flags |=
					match unsafe { mem::transmute::<u8, Encodable>(((dword2 >> D3nowFlags::ENCODABLE_SHIFT) & D3nowFlags::ENCODABLE_MASK) as u8) } {
						Encodable::Any => Flags::MODE16 | Flags::MODE32 | Flags::MODE64,
						Encodable::Only1632 => Flags::MODE16 | Flags::MODE32,
						Encodable::Only64 => Flags::MODE64,
					};
				operand_size = 0;
				address_size = 0;
				l = 0;
				lkind = LKind::None;
			}
		}

		let mut result = Self {
			op_code_string: String::new(),
			instruction_string: String::new(),
			flags,
			code,
			op_code,
			encoding,
			operand_size,
			address_size,
			l,
			tuple_type,
			table,
			mandatory_prefix,
			group_index,
			op0_kind,
			op1_kind,
			op2_kind,
			op3_kind,
			op4_kind,
			lkind,
		};

		let op_code_string = OpCodeFormatter::new(&result, sb, lkind).format();
		result.op_code_string = op_code_string;
		let instruction_string = InstructionFormatter::new(&result, sb).format();
		result.instruction_string = instruction_string;

		result
	}

	/// Gets the code
	///
	/// # Examples
	///
	/// ```
	/// use iced_x86::*;
	///
	/// let op_code = Code::EVEX_Vmovapd_ymm_k1z_ymmm256.op_code();
	/// assert_eq!(Code::EVEX_Vmovapd_ymm_k1z_ymmm256, op_code.code());
	/// ```
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn code(&self) -> Code {
		self.code
	}

	/// Gets the encoding
	///
	/// # Examples
	///
	/// ```
	/// use iced_x86::*;
	///
	/// let op_code = Code::EVEX_Vmovapd_ymm_k1z_ymmm256.op_code();
	/// assert_eq!(EncodingKind::EVEX, op_code.encoding());
	/// ```
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn encoding(&self) -> EncodingKind {
		self.encoding
	}

	/// `true` if it's an instruction, `false` if it's eg. [`Code::INVALID`], [`db`], [`dw`], [`dd`], [`dq`]
	///
	/// # Examples
	///
	/// ```
	/// use iced_x86::*;
	///
	/// assert!(Code::EVEX_Vmovapd_ymm_k1z_ymmm256.op_code().is_instruction());
	/// assert!(!Code::INVALID.op_code().is_instruction());
	/// assert!(!Code::DeclareByte.op_code().is_instruction());
	/// ```
	///
	/// [`Code::INVALID`]: enum.Code.html#variant.INVALID
	/// [`db`]: enum.Code.html#variant.DeclareByte
	/// [`dw`]: enum.Code.html#variant.DeclareWord
	/// [`dd`]: enum.Code.html#variant.DeclareDword
	/// [`dq`]: enum.Code.html#variant.DeclareQword
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn is_instruction(&self) -> bool {
		(self.flags & Flags::NO_INSTRUCTION) == 0
	}

	/// `true` if it's an instruction available in 16-bit mode
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn mode16(&self) -> bool {
		(self.flags & Flags::MODE16) != 0
	}

	/// `true` if it's an instruction available in 32-bit mode
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn mode32(&self) -> bool {
		(self.flags & Flags::MODE32) != 0
	}

	/// `true` if it's an instruction available in 64-bit mode
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn mode64(&self) -> bool {
		(self.flags & Flags::MODE64) != 0
	}

	/// `true` if an `FWAIT` (`9B`) instruction is added before the instruction
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn fwait(&self) -> bool {
		(self.flags & Flags::FWAIT) != 0
	}

	/// (Legacy encoding) Gets the required operand size (16,32,64) or 0 if no operand size prefix (`66`) or `REX.W` prefix is needed
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn operand_size(&self) -> u32 {
		self.operand_size as u32
	}

	/// (Legacy encoding) Gets the required address size (16,32,64) or 0 if no address size prefix (`67`) is needed
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn address_size(&self) -> u32 {
		self.address_size as u32
	}

	/// (VEX/XOP/EVEX) `L` / `L'L` value or default value if [`is_lig()`] is `true`
	///
	/// [`is_lig()`]: #method.is_lig
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn l(&self) -> u32 {
		self.l as u32
	}

	/// (VEX/XOP/EVEX) `W` value or default value if [`is_wig()`] or [`is_wig32()`] is `true`
	///
	/// [`is_wig()`]: #method.is_wig
	/// [`is_wig32()`]: #method.is_wig32
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn w(&self) -> u32 {
		if (self.flags & Flags::W) != 0 {
			1
		} else {
			0
		}
	}

	/// (VEX/XOP/EVEX) `true` if the `L` / `L'L` fields are ignored.
	///
	/// EVEX: if reg-only ops and `{er}` (`EVEX.b` is set), `L'L` is the rounding control and not ignored.
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn is_lig(&self) -> bool {
		(self.flags & Flags::LIG) != 0
	}

	/// (VEX/XOP/EVEX) `true` if the `W` field is ignored in 16/32/64-bit modes
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn is_wig(&self) -> bool {
		(self.flags & Flags::WIG) != 0
	}

	/// (VEX/XOP/EVEX) `true` if the `W` field is ignored in 16/32-bit modes (but not 64-bit mode)
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn is_wig32(&self) -> bool {
		(self.flags & Flags::WIG32) != 0
	}

	/// (EVEX) Gets the tuple type
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn tuple_type(&self) -> TupleType {
		self.tuple_type
	}

	/// (EVEX) `true` if the instruction supports broadcasting (`EVEX.b` bit) (if it has a memory operand)
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn can_broadcast(&self) -> bool {
		(self.flags & Flags::BROADCAST) != 0
	}

	/// (EVEX) `true` if the instruction supports rounding control
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn can_use_rounding_control(&self) -> bool {
		(self.flags & Flags::ROUNDING_CONTROL) != 0
	}

	/// (EVEX) `true` if the instruction supports suppress all exceptions
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn can_suppress_all_exceptions(&self) -> bool {
		(self.flags & Flags::SUPPRESS_ALL_EXCEPTIONS) != 0
	}

	/// (EVEX) `true` if an op mask register can be used
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn can_use_op_mask_register(&self) -> bool {
		(self.flags & Flags::OP_MASK_REGISTER) != 0
	}

	/// (EVEX) `true` if a non-zero op mask register must be used
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn require_non_zero_op_mask_register(&self) -> bool {
		(self.flags & Flags::NON_ZERO_OP_MASK_REGISTER) != 0
	}

	/// (EVEX) `true` if the instruction supports zeroing masking (if one of the op mask registers `K1`-`K7` is used)
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn can_use_zeroing_masking(&self) -> bool {
		(self.flags & Flags::ZEROING_MASKING) != 0
	}

	/// `true` if the `LOCK` (`F0`) prefix can be used
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn can_use_lock_prefix(&self) -> bool {
		(self.flags & Flags::LOCK_PREFIX) != 0
	}

	/// `true` if the `XACQUIRE` (`F2`) prefix can be used
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn can_use_xacquire_prefix(&self) -> bool {
		(self.flags & Flags::XACQUIRE_PREFIX) != 0
	}

	/// `true` if the `XRELEASE` (`F3`) prefix can be used
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn can_use_xrelease_prefix(&self) -> bool {
		(self.flags & Flags::XRELEASE_PREFIX) != 0
	}

	/// `true` if the `REP` / `REPE` (`F3`) prefixes can be used
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn can_use_rep_prefix(&self) -> bool {
		(self.flags & Flags::REP_PREFIX) != 0
	}

	/// `true` if the `REPNE` (`F2`) prefix can be used
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn can_use_repne_prefix(&self) -> bool {
		(self.flags & Flags::REPNE_PREFIX) != 0
	}

	/// `true` if the `BND` (`F2`) prefix can be used
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn can_use_bnd_prefix(&self) -> bool {
		(self.flags & Flags::BND_PREFIX) != 0
	}

	/// `true` if the `HINT-TAKEN` (`3E`) and `HINT-NOT-TAKEN` (`2E`) prefixes can be used
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn can_use_hint_taken_prefix(&self) -> bool {
		(self.flags & Flags::HINT_TAKEN_PREFIX) != 0
	}

	/// `true` if the `NOTRACK` (`3E`) prefix can be used
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn can_use_notrack_prefix(&self) -> bool {
		(self.flags & Flags::NOTRACK_PREFIX) != 0
	}

	/// Gets the opcode table
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn table(&self) -> OpCodeTableKind {
		self.table
	}

	/// Gets the mandatory prefix
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn mandatory_prefix(&self) -> MandatoryPrefix {
		self.mandatory_prefix
	}

	/// Gets the opcode. `000000xxh` if it's 1-byte, `0000yyxxh` if it's 2-byte (`yy` != `00`, and `yy` is the first byte and `xx` the second byte).
	/// It doesn't include the table value, see [`table()`].
	///
	/// # Examples
	///
	/// ```
	/// use iced_x86::*;
	///
	/// assert_eq!(0xDFC0, Code::Ffreep_sti.op_code().op_code());
	/// assert_eq!(0x01D8, Code::Vmrunw.op_code().op_code());
	/// assert_eq!(0x2A, Code::Sub_r8_rm8.op_code().op_code());
	/// assert_eq!(0x2A, Code::Cvtpi2ps_xmm_mmm64.op_code().op_code());
	/// ```
	///
	/// [`table()`]: #method.table
	/// [`Code::Ffreep_sti`]: enum.Code.html#variant.Ffreep_sti
	/// [`Code::Vmrunw`]: enum.Code.html#variant.Vmrunw
	/// [`Code::Sub_r8_rm8`]: enum.Code.html#variant.Sub_r8_rm8
	/// [`Code::Cvtpi2ps_xmm_mmm64`]: enum.Code.html#variant.Cvtpi2ps_xmm_mmm64
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn op_code(&self) -> u32 {
		self.op_code as u32
	}

	/// `true` if it's part of a group
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn is_group(&self) -> bool {
		self.group_index >= 0
	}

	/// Group index (0-7) or -1. If it's 0-7, it's stored in the `reg` field of the `modrm` byte.
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn group_index(&self) -> i32 {
		self.group_index as i32
	}

	/// Gets the number of operands
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn op_count(&self) -> u32 {
		unsafe { *instruction_op_counts::OP_COUNT.get_unchecked(self.code as usize) as u32 }
	}

	/// Gets operand #0's opkind
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn op0_kind(&self) -> OpCodeOperandKind {
		self.op0_kind
	}

	/// Gets operand #1's opkind
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn op1_kind(&self) -> OpCodeOperandKind {
		self.op1_kind
	}

	/// Gets operand #2's opkind
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn op2_kind(&self) -> OpCodeOperandKind {
		self.op2_kind
	}

	/// Gets operand #3's opkind
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn op3_kind(&self) -> OpCodeOperandKind {
		self.op3_kind
	}

	/// Gets operand #4's opkind
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn op4_kind(&self) -> OpCodeOperandKind {
		self.op4_kind
	}

	/// Gets an operand's opkind
	///
	/// # Panics
	///
	/// Panics if `operand` is invalid
	///
	/// # Arguments
	///
	/// * `operand`: Operand number, 0-4
	#[cfg_attr(has_must_use, must_use)]
	#[cfg_attr(feature = "cargo-clippy", allow(clippy::missing_inline_in_public_items))]
	pub fn op_kind(&self, operand: u32) -> OpCodeOperandKind {
		match operand {
			0 => self.op0_kind(),
			1 => self.op1_kind(),
			2 => self.op2_kind(),
			3 => self.op3_kind(),
			4 => self.op4_kind(),
			_ => panic!(),
		}
	}

	/// Checks if the instruction is available in 16-bit mode, 32-bit mode or 64-bit mode
	///
	/// # Panics
	///
	/// Panics if `bitness` is not one of 16, 32, 64.
	///
	/// # Arguments
	///
	/// * `bitness`: 16, 32 or 64
	#[cfg_attr(has_must_use, must_use)]
	#[cfg_attr(feature = "cargo-clippy", allow(clippy::missing_inline_in_public_items))]
	pub fn is_available_in_mode(&self, bitness: u32) -> bool {
		match bitness {
			16 => self.mode16(),
			32 => self.mode32(),
			64 => self.mode64(),
			_ => panic!(),
		}
	}

	/// Gets the opcode string, eg. `VEX.128.66.0F38.W0 78 /r`, see also [`instruction_string()`]
	///
	/// [`instruction_string()`]: #method.instruction_string
	///
	/// # Examples
	///
	/// ```
	/// use iced_x86::*;
	///
	/// let op_code = Code::EVEX_Vmovapd_ymm_k1z_ymmm256.op_code();
	/// assert_eq!("EVEX.256.66.0F.W1 28 /r", op_code.op_code_string());
	/// ```
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn op_code_string(&self) -> &str {
		self.op_code_string.as_str()
	}

	/// Gets the instruction string, eg. `VPBROADCASTB xmm1, xmm2/m8`, see also [`op_code_string()`]
	///
	/// [`op_code_string()`]: #method.op_code_string
	///
	/// # Examples
	///
	/// ```
	/// use iced_x86::*;
	///
	/// let op_code = Code::EVEX_Vmovapd_ymm_k1z_ymmm256.op_code();
	/// assert_eq!("VMOVAPD ymm1 {k1}{z}, ymm2/m256", op_code.instruction_string());
	/// ```
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	pub fn instruction_string(&self) -> &str {
		self.instruction_string.as_str()
	}
}

impl fmt::Display for OpCodeInfo {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", self.instruction_string)
	}
}
