/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use super::super::iced_constants::IcedConstants;
use super::super::*;
use super::mnemonic_str_tbl::TO_MNEMONIC_STR;
use super::op_code::OpCodeInfo;
#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::char;
use core::fmt::Write;

pub(super) struct InstructionFormatter<'a, 'b> {
	op_code: &'a OpCodeInfo,
	sb: &'b mut String,
	r32_count: u32,
	r64_count: u32,
	bnd_count: u32,
	start_op_index: u32,
	r32_index: u32,
	r64_index: u32,
	bnd_index: u32,
	k_index: u32,
	vec_index: u32,
	tmm_index: u32,
	op_count: u32,
	// true: k2 {k1}, false: k1 {k2}
	op_mask_is_k1: bool,
	no_vec_index: bool,
	swap_vec_index_12: bool,
	no_gpr_suffix: bool,
}

impl<'a, 'b> InstructionFormatter<'a, 'b> {
	#[allow(unused_mut)]
	pub(super) fn new(op_code: &'a OpCodeInfo, fmt_opt: InstrStrFmtOption, sb: &'b mut String) -> InstructionFormatter<'a, 'b> {
		let mut no_vec_index = false;
		let mut swap_vec_index_12 = false;
		let mut no_gpr_suffix = false;
		let mut start_op_index = 0;
		let mut bnd_count = 0;
		let mut r32_count = 0;
		let mut r64_count = 0;
		let r32_index = 0;
		let r64_index = 0;
		let k_index = 0;
		let mut vec_index = 0;
		let tmm_index = 0;
		let bnd_index = 0;
		let mut op_count = op_code.op_count();
		let mut op_mask_is_k1 = false;
		match fmt_opt {
			InstrStrFmtOption::None => {}
			InstrStrFmtOption::OpMaskIsK1_or_NoGprSuffix => {
				op_mask_is_k1 = true;
				no_gpr_suffix = true;
			}
			InstrStrFmtOption::IncVecIndex => vec_index += 1,
			InstrStrFmtOption::NoVecIndex => no_vec_index = true,
			InstrStrFmtOption::SwapVecIndex12 => swap_vec_index_12 = true,
			InstrStrFmtOption::SkipOp0 => start_op_index = 1,
		}
		if (op_code.op0_kind() == OpCodeOperandKind::k_reg || op_code.op0_kind() == OpCodeOperandKind::kp1_reg) && op_code.op_count() > 2 {
			vec_index += 1;
		}
		for i in 0..op_code.op_count() {
			match op_code.op_kind(i) {
				OpCodeOperandKind::r32_reg
				| OpCodeOperandKind::r32_reg_mem
				| OpCodeOperandKind::r32_rm
				| OpCodeOperandKind::r32_opcode
				| OpCodeOperandKind::r32_vvvv => r32_count += 1,

				OpCodeOperandKind::r64_reg
				| OpCodeOperandKind::r64_reg_mem
				| OpCodeOperandKind::r64_rm
				| OpCodeOperandKind::r64_opcode
				| OpCodeOperandKind::r64_vvvv => r64_count += 1,

				OpCodeOperandKind::bnd_or_mem_mpx | OpCodeOperandKind::bnd_reg => bnd_count += 1,

				OpCodeOperandKind::None
				| OpCodeOperandKind::farbr2_2
				| OpCodeOperandKind::farbr4_2
				| OpCodeOperandKind::mem_offs
				| OpCodeOperandKind::mem
				| OpCodeOperandKind::mem_mpx
				| OpCodeOperandKind::mem_mib
				| OpCodeOperandKind::mem_vsib32x
				| OpCodeOperandKind::mem_vsib64x
				| OpCodeOperandKind::mem_vsib32y
				| OpCodeOperandKind::mem_vsib64y
				| OpCodeOperandKind::mem_vsib32z
				| OpCodeOperandKind::mem_vsib64z
				| OpCodeOperandKind::r8_or_mem
				| OpCodeOperandKind::r16_or_mem
				| OpCodeOperandKind::r32_or_mem
				| OpCodeOperandKind::r32_or_mem_mpx
				| OpCodeOperandKind::r64_or_mem
				| OpCodeOperandKind::r64_or_mem_mpx
				| OpCodeOperandKind::mm_or_mem
				| OpCodeOperandKind::xmm_or_mem
				| OpCodeOperandKind::ymm_or_mem
				| OpCodeOperandKind::zmm_or_mem
				| OpCodeOperandKind::k_or_mem
				| OpCodeOperandKind::r8_reg
				| OpCodeOperandKind::r8_opcode
				| OpCodeOperandKind::r16_reg
				| OpCodeOperandKind::r16_reg_mem
				| OpCodeOperandKind::r16_rm
				| OpCodeOperandKind::r16_opcode
				| OpCodeOperandKind::seg_reg
				| OpCodeOperandKind::k_reg
				| OpCodeOperandKind::kp1_reg
				| OpCodeOperandKind::k_rm
				| OpCodeOperandKind::k_vvvv
				| OpCodeOperandKind::mm_reg
				| OpCodeOperandKind::mm_rm
				| OpCodeOperandKind::xmm_reg
				| OpCodeOperandKind::xmm_rm
				| OpCodeOperandKind::xmm_vvvv
				| OpCodeOperandKind::xmmp3_vvvv
				| OpCodeOperandKind::xmm_is4
				| OpCodeOperandKind::xmm_is5
				| OpCodeOperandKind::ymm_reg
				| OpCodeOperandKind::ymm_rm
				| OpCodeOperandKind::ymm_vvvv
				| OpCodeOperandKind::ymm_is4
				| OpCodeOperandKind::ymm_is5
				| OpCodeOperandKind::zmm_reg
				| OpCodeOperandKind::zmm_rm
				| OpCodeOperandKind::zmm_vvvv
				| OpCodeOperandKind::zmmp3_vvvv
				| OpCodeOperandKind::cr_reg
				| OpCodeOperandKind::dr_reg
				| OpCodeOperandKind::tr_reg
				| OpCodeOperandKind::es
				| OpCodeOperandKind::cs
				| OpCodeOperandKind::ss
				| OpCodeOperandKind::ds
				| OpCodeOperandKind::fs
				| OpCodeOperandKind::gs
				| OpCodeOperandKind::al
				| OpCodeOperandKind::cl
				| OpCodeOperandKind::ax
				| OpCodeOperandKind::dx
				| OpCodeOperandKind::eax
				| OpCodeOperandKind::rax
				| OpCodeOperandKind::st0
				| OpCodeOperandKind::sti_opcode
				| OpCodeOperandKind::imm2_m2z
				| OpCodeOperandKind::imm8
				| OpCodeOperandKind::imm8_const_1
				| OpCodeOperandKind::imm8sex16
				| OpCodeOperandKind::imm8sex32
				| OpCodeOperandKind::imm8sex64
				| OpCodeOperandKind::imm16
				| OpCodeOperandKind::imm32
				| OpCodeOperandKind::imm32sex64
				| OpCodeOperandKind::imm64
				| OpCodeOperandKind::seg_rDI
				| OpCodeOperandKind::br16_1
				| OpCodeOperandKind::br32_1
				| OpCodeOperandKind::br64_1
				| OpCodeOperandKind::br16_2
				| OpCodeOperandKind::br32_4
				| OpCodeOperandKind::br64_4
				| OpCodeOperandKind::xbegin_2
				| OpCodeOperandKind::xbegin_4
				| OpCodeOperandKind::brdisp_2
				| OpCodeOperandKind::brdisp_4
				| OpCodeOperandKind::sibmem
				| OpCodeOperandKind::tmm_reg
				| OpCodeOperandKind::tmm_rm
				| OpCodeOperandKind::tmm_vvvv => {}

				OpCodeOperandKind::seg_rSI | OpCodeOperandKind::es_rDI | OpCodeOperandKind::seg_rBX_al => {
					// string instructions, xlat
					op_count = 0;
				}
			}
		}

		Self {
			op_code,
			sb,
			r32_count,
			r64_count,
			bnd_count,
			start_op_index,
			r32_index,
			r64_index,
			bnd_index,
			k_index,
			vec_index,
			tmm_index,
			op_count,
			op_mask_is_k1,
			no_vec_index,
			swap_vec_index_12,
			no_gpr_suffix,
		}
	}

	fn get_k_index(&mut self) -> u32 {
		self.k_index += 1;
		if self.op_mask_is_k1 {
			if self.k_index == 1 {
				return 2;
			}
			if self.k_index == 2 {
				return 1;
			}
		}
		self.k_index
	}

	fn get_bnd_index(&mut self) -> u32 {
		if self.bnd_count <= 1 {
			0
		} else {
			self.bnd_index += 1;
			self.bnd_index
		}
	}

	fn get_vec_index(&mut self) -> u32 {
		if self.no_vec_index {
			return 0;
		}
		self.vec_index += 1;
		if self.swap_vec_index_12 {
			if self.vec_index == 1 {
				return 2;
			}
			if self.vec_index == 2 {
				return 1;
			}
		}
		self.vec_index
	}

	fn get_tmm_index(&mut self) -> u32 {
		self.tmm_index += 1;
		self.tmm_index
	}

	fn get_memory_size(&self, is_broadcast: bool) -> MemorySize {
		let mut index = self.op_code.code() as usize;
		if is_broadcast {
			index += IcedConstants::NUMBER_OF_CODE_VALUES;
		}
		instruction_memory_sizes::SIZES[index]
	}

	pub(super) fn format(&mut self) -> String {
		if !self.op_code.is_instruction() {
			match self.op_code.code() {
				// GENERATOR-BEGIN: InstrFmtNotInstructionString
				// ⚠️This was generated by GENERATOR!🦹‍♂️
				Code::INVALID => return String::from("<invalid>"),
				Code::DeclareByte => return String::from("<db>"),
				Code::DeclareWord => return String::from("<dw>"),
				Code::DeclareDword => return String::from("<dd>"),
				Code::DeclareQword => return String::from("<dq>"),
				// GENERATOR-END: InstrFmtNotInstructionString
				_ => unreachable!(),
			}
		}

		self.sb.clear();

		// Temp needed if rustc < 1.36.0 (2015 edition)
		let tmp_mnemonic = TO_MNEMONIC_STR[self.op_code.code().mnemonic() as usize];
		self.write(tmp_mnemonic, true);
		if self.start_op_index < self.op_count {
			self.sb.push(' ');
			let mut sae_er_index = self.op_count - 1;
			if self.op_code.encoding() != EncodingKind::Legacy && self.op_code.op_kind(sae_er_index) == OpCodeOperandKind::imm8 {
				sae_er_index -= 1;
			}
			let mut add_comma = false;
			for i in self.start_op_index..self.op_count {
				let mut tmp;
				let tmp2;
				if add_comma {
					self.write_op_separator();
				}
				add_comma = true;

				let op_kind = self.op_code.op_kind(i);
				match op_kind {
					OpCodeOperandKind::farbr2_2 => self.sb.push_str("ptr16:16"),
					OpCodeOperandKind::farbr4_2 => self.sb.push_str("ptr16:32"),

					OpCodeOperandKind::mem_offs => {
						self.sb.push_str("moffs");
						// Temp needed if rustc < 1.36.0 (2015 edition)
						let tmp_mem_size = self.get_memory_size(false);
						self.write_memory_size(tmp_mem_size);
					}

					OpCodeOperandKind::mem | OpCodeOperandKind::mem_mpx => self.write_memory(),
					OpCodeOperandKind::sibmem => self.sb.push_str("sibmem"),
					OpCodeOperandKind::mem_mib => self.sb.push_str("mib"),

					OpCodeOperandKind::mem_vsib32x => self.sb.push_str("vm32x"),
					OpCodeOperandKind::mem_vsib64x => self.sb.push_str("vm64x"),
					OpCodeOperandKind::mem_vsib32y => self.sb.push_str("vm32y"),
					OpCodeOperandKind::mem_vsib64y => self.sb.push_str("vm64y"),
					OpCodeOperandKind::mem_vsib32z => self.sb.push_str("vm32z"),
					OpCodeOperandKind::mem_vsib64z => self.sb.push_str("vm64z"),
					OpCodeOperandKind::r8_or_mem => self.write_gpr_mem(8),
					OpCodeOperandKind::r16_or_mem => self.write_gpr_mem(16),
					OpCodeOperandKind::r32_or_mem | OpCodeOperandKind::r32_or_mem_mpx => self.write_gpr_mem(32),
					OpCodeOperandKind::r64_or_mem | OpCodeOperandKind::r64_or_mem_mpx => self.write_gpr_mem(64),

					OpCodeOperandKind::mm_or_mem => {
						tmp = self.get_vec_index();
						self.write_reg_mem("mm", tmp);
					}

					OpCodeOperandKind::xmm_or_mem => {
						tmp = self.get_vec_index();
						self.write_reg_mem("xmm", tmp);
					}

					OpCodeOperandKind::ymm_or_mem => {
						tmp = self.get_vec_index();
						self.write_reg_mem("ymm", tmp);
					}

					OpCodeOperandKind::zmm_or_mem => {
						tmp = self.get_vec_index();
						self.write_reg_mem("zmm", tmp);
					}

					OpCodeOperandKind::bnd_or_mem_mpx => {
						tmp = self.get_bnd_index();
						self.write_reg_op2("bnd", tmp);
						self.sb.push('/');
						self.write_memory();
					}

					OpCodeOperandKind::k_or_mem => {
						tmp = self.get_k_index();
						self.write_reg_mem("k", tmp);
					}

					OpCodeOperandKind::r8_reg | OpCodeOperandKind::r8_opcode => self.write_reg_op1("r8"),
					OpCodeOperandKind::r16_reg | OpCodeOperandKind::r16_reg_mem | OpCodeOperandKind::r16_rm | OpCodeOperandKind::r16_opcode => {
						self.write_reg_op1("r16")
					}

					OpCodeOperandKind::r32_reg
					| OpCodeOperandKind::r32_reg_mem
					| OpCodeOperandKind::r32_rm
					| OpCodeOperandKind::r32_opcode
					| OpCodeOperandKind::r32_vvvv => {
						self.write_reg_op1("r32");
						tmp2 = self.r32_count;
						tmp = self.r32_index;
						self.append_gpr_suffix(tmp2, &mut tmp);
						self.r32_index = tmp;
					}

					OpCodeOperandKind::r64_reg
					| OpCodeOperandKind::r64_reg_mem
					| OpCodeOperandKind::r64_rm
					| OpCodeOperandKind::r64_opcode
					| OpCodeOperandKind::r64_vvvv => {
						self.write_reg_op1("r64");
						tmp2 = self.r64_count;
						tmp = self.r64_index;
						self.append_gpr_suffix(tmp2, &mut tmp);
						self.r64_index = tmp;
					}

					OpCodeOperandKind::seg_reg => self.sb.push_str("Sreg"),
					OpCodeOperandKind::k_reg | OpCodeOperandKind::k_rm | OpCodeOperandKind::k_vvvv => {
						tmp = self.get_k_index();
						self.write_reg_op2("k", tmp);
					}

					OpCodeOperandKind::kp1_reg => {
						tmp = self.get_k_index();
						self.write_reg_op2("k", tmp);
						self.sb.push_str("+1");
					}

					OpCodeOperandKind::mm_reg | OpCodeOperandKind::mm_rm => {
						tmp = self.get_vec_index();
						self.write_reg_op2("mm", tmp);
					}

					OpCodeOperandKind::xmm_reg
					| OpCodeOperandKind::xmm_rm
					| OpCodeOperandKind::xmm_vvvv
					| OpCodeOperandKind::xmm_is4
					| OpCodeOperandKind::xmm_is5 => {
						tmp = self.get_vec_index();
						self.write_reg_op2("xmm", tmp);
					}

					OpCodeOperandKind::xmmp3_vvvv => {
						tmp = self.get_vec_index();
						self.write_reg_op2("xmm", tmp);
						self.sb.push_str("+3");
					}

					OpCodeOperandKind::ymm_reg
					| OpCodeOperandKind::ymm_rm
					| OpCodeOperandKind::ymm_vvvv
					| OpCodeOperandKind::ymm_is4
					| OpCodeOperandKind::ymm_is5 => {
						tmp = self.get_vec_index();
						self.write_reg_op2("ymm", tmp);
					}

					OpCodeOperandKind::zmm_reg | OpCodeOperandKind::zmm_rm | OpCodeOperandKind::zmm_vvvv => {
						tmp = self.get_vec_index();
						self.write_reg_op2("zmm", tmp);
					}

					OpCodeOperandKind::zmmp3_vvvv => {
						tmp = self.get_vec_index();
						self.write_reg_op2("zmm", tmp);
						self.sb.push_str("+3");
					}

					OpCodeOperandKind::tmm_reg | OpCodeOperandKind::tmm_rm | OpCodeOperandKind::tmm_vvvv => {
						tmp = self.get_tmm_index();
						self.write_reg_op2("tmm", tmp);
					}

					OpCodeOperandKind::bnd_reg => {
						tmp = self.get_bnd_index();
						self.write_reg_op2("bnd", tmp);
					}

					OpCodeOperandKind::cr_reg => self.write_reg_op1("cr"),
					OpCodeOperandKind::dr_reg => self.write_reg_op1("dr"),
					OpCodeOperandKind::tr_reg => self.write_reg_op1("tr"),
					OpCodeOperandKind::es => self.write_register("es"),
					OpCodeOperandKind::cs => self.write_register("cs"),
					OpCodeOperandKind::ss => self.write_register("ss"),
					OpCodeOperandKind::ds => self.write_register("ds"),
					OpCodeOperandKind::fs => self.write_register("fs"),
					OpCodeOperandKind::gs => self.write_register("gs"),
					OpCodeOperandKind::al => self.write_register("al"),
					OpCodeOperandKind::cl => self.write_register("cl"),
					OpCodeOperandKind::ax => self.write_register("ax"),
					OpCodeOperandKind::dx => self.write_register("dx"),
					OpCodeOperandKind::eax => self.write_register("eax"),
					OpCodeOperandKind::rax => self.write_register("rax"),

					OpCodeOperandKind::st0 | OpCodeOperandKind::sti_opcode => {
						self.write_register("ST");
						if op_kind == OpCodeOperandKind::st0 {
							match self.op_code.code() {
								Code::Fcomi_st0_sti | Code::Fcomip_st0_sti | Code::Fucomi_st0_sti | Code::Fucomip_st0_sti => {}
								_ => self.sb.push_str("(0)"),
							}
						} else {
							debug_assert_eq!(OpCodeOperandKind::sti_opcode, op_kind);
							self.sb.push_str("(i)");
						}
					}

					OpCodeOperandKind::imm2_m2z => self.sb.push_str("imm2"),

					OpCodeOperandKind::imm8 | OpCodeOperandKind::imm8sex16 | OpCodeOperandKind::imm8sex32 | OpCodeOperandKind::imm8sex64 => {
						self.sb.push_str("imm8")
					}
					OpCodeOperandKind::imm8_const_1 => self.sb.push_str("1"),
					OpCodeOperandKind::imm16 => self.sb.push_str("imm16"),
					OpCodeOperandKind::imm32 | OpCodeOperandKind::imm32sex64 => self.sb.push_str("imm32"),
					OpCodeOperandKind::imm64 => self.sb.push_str("imm64"),

					OpCodeOperandKind::seg_rSI | OpCodeOperandKind::es_rDI | OpCodeOperandKind::seg_rDI | OpCodeOperandKind::seg_rBX_al => {
						add_comma = false
					}

					OpCodeOperandKind::br16_1 | OpCodeOperandKind::br32_1 | OpCodeOperandKind::br64_1 => self.sb.push_str("rel8"),
					OpCodeOperandKind::br16_2 | OpCodeOperandKind::xbegin_2 => self.sb.push_str("rel16"),
					OpCodeOperandKind::br32_4 | OpCodeOperandKind::br64_4 | OpCodeOperandKind::xbegin_4 => self.sb.push_str("rel32"),
					OpCodeOperandKind::brdisp_2 => self.sb.push_str("disp16"),
					OpCodeOperandKind::brdisp_4 => self.sb.push_str("disp32"),
					OpCodeOperandKind::None => unreachable!(),
				}

				if i == 0 {
					if self.op_code.can_use_op_mask_register() {
						self.sb.push(' ');
						tmp = self.get_k_index();
						self.write_reg_decorator("k", tmp);
						if self.op_code.can_use_zeroing_masking() {
							self.write_decorator("z");
						}
					}
				}
				if i == sae_er_index {
					if self.op_code.can_suppress_all_exceptions() {
						self.write_decorator("sae");
					}
					if self.op_code.can_use_rounding_control() {
						self.write_decorator("er");
					}
				}
			}
		}

		match self.op_code.code() {
			Code::Blendvpd_xmm_xmmm128 | Code::Blendvps_xmm_xmmm128 | Code::Pblendvb_xmm_xmmm128 | Code::Sha256rnds2_xmm_xmmm128 => {
				self.write_op_separator();
				self.write("<XMM0>", true);
			}

			Code::Tpause_r32 | Code::Tpause_r64 | Code::Umwait_r32 | Code::Umwait_r64 => {
				self.write_op_separator();
				self.write("<edx>", false);
				self.write_op_separator();
				self.write("<eax>", false);
			}

			Code::Aesencwide128kl_m384 | Code::Aesdecwide128kl_m384 | Code::Aesencwide256kl_m512 | Code::Aesdecwide256kl_m512 => {
				self.write_op_separator();
				self.write("<XMM0-7>", true);
			}

			Code::Loadiwkey_xmm_xmm => {
				self.write_op_separator();
				self.write("<eax>", true);
				self.write_op_separator();
				self.write("<XMM0>", true);
			}

			Code::Encodekey128_r32_r32 | Code::Encodekey128_r64_r64 => {
				self.write_op_separator();
				self.write("<XMM0-2>", true);
				self.write_op_separator();
				self.write("<XMM4-6>", true);
			}

			Code::Encodekey256_r32_r32 | Code::Encodekey256_r64_r64 => {
				self.write_op_separator();
				self.write("<XMM0-6>", true);
			}

			Code::Hreset_imm8 => {
				self.write_op_separator();
				self.write("<eax>", true);
			}

			_ => {}
		}

		self.sb.clone()
	}

	fn write_memory_size(&mut self, memory_size: MemorySize) {
		match self.op_code.code() {
			Code::Fldcw_m2byte | Code::Fnstcw_m2byte | Code::Fstcw_m2byte | Code::Fnstsw_m2byte | Code::Fstsw_m2byte => {
				self.sb.push_str("2byte");
				return;
			}
			_ => {}
		}

		match memory_size {
			MemorySize::Bound16_WordWord => self.sb.push_str("16&16"),
			MemorySize::Bound32_DwordDword => self.sb.push_str("32&32"),
			MemorySize::FpuEnv14 => self.sb.push_str("14byte"),
			MemorySize::FpuEnv28 => self.sb.push_str("28byte"),
			MemorySize::FpuState94 => self.sb.push_str("94byte"),
			MemorySize::FpuState108 => self.sb.push_str("108byte"),
			MemorySize::Fxsave_512Byte | MemorySize::Fxsave64_512Byte => self.sb.push_str("512byte"),
			MemorySize::Xsave | MemorySize::Xsave64 => self.sb.push_str("em"), // 'm' has already been appended
			MemorySize::SegPtr16 => self.sb.push_str("16:16"),
			MemorySize::SegPtr32 => self.sb.push_str("16:32"),
			MemorySize::SegPtr64 => self.sb.push_str("16:64"),

			MemorySize::Fword6 => {
				if !self.is_sgdt_or_sidt() {
					self.sb.push_str("16&32");
				}
			}

			MemorySize::Fword10 => {
				if !self.is_sgdt_or_sidt() {
					self.sb.push_str("16&64");
				}
			}

			_ => {
				let mem_size = memory_size.size();
				if mem_size != 0 {
					write!(self.sb, "{}", mem_size * 8).unwrap();
				}
			}
		}

		if Self::is_fpu_instruction(self.op_code.code()) {
			match memory_size {
				MemorySize::Int16 | MemorySize::Int32 | MemorySize::Int64 => self.sb.push_str("int"),
				MemorySize::Float32 | MemorySize::Float64 | MemorySize::Float80 => self.sb.push_str("fp"),
				MemorySize::Bcd => self.sb.push_str("bcd"),
				_ => {}
			}
		}
	}

	fn is_sgdt_or_sidt(&self) -> bool {
		match self.op_code.code() {
			Code::Sgdt_m1632_16 | Code::Sgdt_m1632 | Code::Sgdt_m1664 | Code::Sidt_m1632_16 | Code::Sidt_m1632 | Code::Sidt_m1664 => true,
			_ => false,
		}
	}

	fn write_register(&mut self, register: &str) {
		self.write(register, true);
	}

	fn write_reg_op1(&mut self, register: &str) {
		self.write(register, false);
	}

	fn write_reg_op2(&mut self, register: &str, index: u32) {
		self.write_reg_op1(register);
		if index > 0 {
			write!(self.sb, "{}", index).unwrap();
		}
	}

	fn write_decorator(&mut self, decorator: &str) {
		self.sb.push('{');
		self.write(decorator, false);
		self.sb.push('}');
	}

	fn write_reg_decorator(&mut self, register: &str, index: u32) {
		self.sb.push('{');
		self.write(register, false);
		write!(self.sb, "{}", index).unwrap();
		self.sb.push('}');
	}

	fn append_gpr_suffix(&mut self, count: u32, index: &mut u32) {
		if count <= 1 || self.no_gpr_suffix {
			return;
		}
		self.sb.push(char::from_u32('a' as u32 + *index).unwrap());
		*index += 1;
	}

	fn write_op_separator(&mut self) {
		self.sb.push_str(", ");
	}

	fn write(&mut self, s: &str, upper: bool) {
		if upper {
			for c in s.chars() {
				for uc in c.to_uppercase() {
					self.sb.push(uc);
				}
			}
		} else {
			for c in s.chars() {
				for lc in c.to_lowercase() {
					self.sb.push(lc);
				}
			}
		}
	}

	fn write_gpr_mem(&mut self, reg_size: usize) {
		debug_assert!(!self.op_code.can_broadcast());
		self.sb.push('r');
		let mem_size = self.get_memory_size(false).size() * 8;
		if mem_size != reg_size {
			write!(self.sb, "{}", reg_size).unwrap();
		}
		self.sb.push('/');
		self.write_memory();
	}

	fn write_reg_mem(&mut self, register: &str, index: u32) {
		self.write_reg_op2(register, index);
		self.sb.push('/');
		self.write_memory();
	}

	fn write_memory(&mut self) {
		self.write_memory1(false);
		if self.op_code.can_broadcast() {
			self.sb.push('/');
			self.write_memory1(true);
		}
	}

	fn write_memory1(&mut self, is_broadcast: bool) {
		let memory_size = self.get_memory_size(is_broadcast);
		self.sb.push('m');
		self.write_memory_size(memory_size);
		if is_broadcast {
			self.sb.push_str("bcst");
		}
	}

	fn is_fpu_instruction(code: Code) -> bool {
		(code as u32).wrapping_sub(Code::Fadd_m32fp as u32) <= (Code::Fcomip_st0_sti as u32 - Code::Fadd_m32fp as u32)
	}
}
