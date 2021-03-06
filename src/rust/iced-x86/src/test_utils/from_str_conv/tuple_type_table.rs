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

#![allow(unused_results)]

use super::super::super::TupleType;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

lazy_static! {
	pub(super) static ref TO_TUPLE_TYPE_HASH: HashMap<&'static str, TupleType> = {
		// GENERATOR-BEGIN: TupleTypeHash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(14);
		h.insert("N1", TupleType::N1);
		h.insert("N2", TupleType::N2);
		h.insert("N4", TupleType::N4);
		h.insert("N8", TupleType::N8);
		h.insert("N16", TupleType::N16);
		h.insert("N32", TupleType::N32);
		h.insert("N64", TupleType::N64);
		h.insert("N8b4", TupleType::N8b4);
		h.insert("N16b4", TupleType::N16b4);
		h.insert("N32b4", TupleType::N32b4);
		h.insert("N64b4", TupleType::N64b4);
		h.insert("N16b8", TupleType::N16b8);
		h.insert("N32b8", TupleType::N32b8);
		h.insert("N64b8", TupleType::N64b8);
		// GENERATOR-END: TupleTypeHash
		h
	};
}
