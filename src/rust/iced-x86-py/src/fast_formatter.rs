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

use crate::instruction::Instruction;
use pyo3::prelude::*;

/// Fast formatter with less formatting options and with a masm-like syntax.
///
/// Use it if formatting speed is more important than being able to re-assemble formatted instructions.
///
/// This formatter is ~1.25x faster than the other formatters (the time includes decoding + formatting).
///
/// Examples:
///
/// .. testcode::
///
///     from iced_x86 import *
///
///     data = b"\x62\xF2\x4F\xDD\x72\x50\x01"
///     decoder = Decoder(64, data)
///     instr = decoder.decode()
///
///     formatter = FastFormatter()
///     formatter.space_after_operand_separator = True
///     disasm = formatter.format(instr)
///     assert disasm == "vcvtne2ps2bf16 zmm2{k5}{z}, zmm6, dword bcst [rax+4h]"
#[pyclass(module = "_iced_x86_py")]
#[text_signature = "(/)"]
pub struct FastFormatter {
	fmt_output: String,
	formatter: iced_x86::FastFormatter,
}

unsafe impl Send for FastFormatter {}

#[pymethods]
impl FastFormatter {
	#[new]
	fn new() -> Self {
		Self { fmt_output: String::new(), formatter: iced_x86::FastFormatter::new() }
	}

	/// Formats the whole instruction: prefixes, mnemonic, operands
	///
	/// Args:
	///     `instruction` (Instruction): Instruction to format
	///
	/// Returns:
	///     str: The formatted string
	#[text_signature = "($self, instruction)"]
	fn format(&mut self, instruction: &Instruction) -> &str {
		self.fmt_output.clear();
		self.formatter.format(&instruction.instr, &mut self.fmt_output);
		&self.fmt_output
	}

	/// bool: Add a space after the operand separator
	///
	/// =========== ========== ================================================
	/// Default     Value      Example
	/// =========== ========== ================================================
	/// \           ``True``   ``mov rax, rcx``
	/// ✔️          ``False``   ``mov rax,rcx``
	/// =========== ========== ================================================
	#[getter]
	fn space_after_operand_separator(&self) -> bool {
		self.formatter.options().space_after_operand_separator()
	}

	#[setter]
	fn set_space_after_operand_separator(&mut self, value: bool) {
		self.formatter.options_mut().set_space_after_operand_separator(value);
	}

	/// bool: Show ``RIP+displ`` or the virtual address
	///
	/// =========== ========== ================================================
	/// Default     Value      Example
	/// =========== ========== ================================================
	/// \           ``True``   ``mov eax,[rip+12345678h]``
	/// ✔️          ``False``   ``mov eax,[1029384756AFBECDh]``
	/// =========== ========== ================================================
	#[getter]
	fn rip_relative_addresses(&self) -> bool {
		self.formatter.options().rip_relative_addresses()
	}

	#[setter]
	fn set_rip_relative_addresses(&mut self, value: bool) {
		self.formatter.options_mut().set_rip_relative_addresses(value);
	}

	/// bool: Use pseudo instructions
	///
	/// =========== ========== ================================================
	/// Default     Value      Example
	/// =========== ========== ================================================
	/// ✔️          ``True``   ``vcmpnltsd xmm2,xmm6,xmm3``
	/// \           ``False``   ``vcmpsd xmm2,xmm6,xmm3,5``
	/// =========== ========== ================================================
	#[getter]
	fn use_pseudo_ops(&self) -> bool {
		self.formatter.options().use_pseudo_ops()
	}

	#[setter]
	fn set_use_pseudo_ops(&mut self, value: bool) {
		self.formatter.options_mut().set_use_pseudo_ops(value);
	}

	/// bool: Show the original value after the symbol name
	///
	/// =========== ========== ================================================
	/// Default     Value      Example
	/// =========== ========== ================================================
	/// \           ``True``   ``mov eax,[myfield (12345678)]``
	/// ✔️          ``False``   ``mov eax,[myfield]``
	/// =========== ========== ================================================
	#[getter]
	fn show_symbol_address(&self) -> bool {
		self.formatter.options().show_symbol_address()
	}

	#[setter]
	fn set_show_symbol_address(&mut self, value: bool) {
		self.formatter.options_mut().set_show_symbol_address(value);
	}

	/// bool: Always show the effective segment register.
	///
	/// If the option is ``False``, only show the segment register if there's a segment override prefix.
	///
	/// =========== ========== ================================================
	/// Default     Value      Example
	/// =========== ========== ================================================
	/// \           ``True``   ``mov eax,ds:[ecx]``
	/// ✔️          ``False``   ``mov eax,[ecx]``
	/// =========== ========== ================================================
	#[getter]
	fn always_show_segment_register(&self) -> bool {
		self.formatter.options().always_show_segment_register()
	}

	#[setter]
	fn set_always_show_segment_register(&mut self, value: bool) {
		self.formatter.options_mut().set_always_show_segment_register(value);
	}

	/// bool: Always show the size of memory operands
	///
	/// =========== ========== ============================= ===================
	/// Default     Value      Example                       Example
	/// =========== ========== ============================= ===================
	/// \           ``True``   ``mov eax,dword ptr [ebx]``   ``add byte ptr [eax],0x12``
	/// ✔️          ``False``   ``mov eax,[ebx]``            ``add byte ptr [eax],0x12``
	/// =========== ========== ============================= ===================
	#[getter]
	fn always_show_memory_size(&self) -> bool {
		self.formatter.options().always_show_memory_size()
	}

	#[setter]
	fn set_always_show_memory_size(&mut self, value: bool) {
		self.formatter.options_mut().set_always_show_memory_size(value)
	}

	/// bool: Use upper case hex digits
	///
	/// =========== ========== ================================================
	/// Default     Value      Example
	/// =========== ========== ================================================
	/// ✔️          ``True``   ``0xFF``
	/// \           ``False``   ``0xff``
	/// =========== ========== ================================================
	#[getter]
	fn uppercase_hex(&self) -> bool {
		self.formatter.options().uppercase_hex()
	}

	#[setter]
	fn set_uppercase_hex(&mut self, value: bool) {
		self.formatter.options_mut().set_uppercase_hex(value);
	}

	/// bool: Use a hex prefix (``0x``) or a hex suffix (``h``)
	///
	/// =========== ========== ================================================
	/// Default     Value      Example
	/// =========== ========== ================================================
	/// \           ``True``   ``0x5A``
	/// ✔️          ``False``   ``5Ah``
	/// =========== ========== ================================================
	#[getter]
	fn use_hex_prefix(&self) -> bool {
		self.formatter.options().use_hex_prefix()
	}

	#[setter]
	fn set_use_hex_prefix(&mut self, value: bool) {
		self.formatter.options_mut().set_use_hex_prefix(value)
	}
}
