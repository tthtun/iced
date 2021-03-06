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

using System;
using System.Collections.Generic;
using Generator.Documentation;
using Generator.Documentation.Python;
using Generator.IO;

namespace Generator.Enums.Python {
	[Generator(TargetLanguage.Python)]
	sealed class PythonEnumsGenerator : EnumsGenerator {
		readonly IdentifierConverter pythonIdConverter;
		readonly IdentifierConverter rustIdConverter;
		readonly Dictionary<TypeId, FullEnumFileInfo?> toFullFileInfo;
		readonly Dictionary<TypeId, PartialEnumFileInfo?> toPartialFileInfo;
		readonly Documentation.Rust.RustDocCommentWriter rustDocWriter;

		sealed class FullEnumFileInfo {
			public readonly string Filename;

			public FullEnumFileInfo(string filename) => Filename = filename;
		}

		sealed class PartialEnumFileInfo {
			public readonly string Id;
			public readonly TargetLanguage Language;
			public readonly string Filename;
			public readonly string[] Attributes;

			public PartialEnumFileInfo(string id, TargetLanguage language, string filename, params string[] attributes) {
				Id = id;
				Language = language;
				Filename = filename;
				Attributes = attributes;
			}
		}

		public PythonEnumsGenerator(GeneratorContext generatorContext)
			: base(generatorContext.Types) {
			pythonIdConverter = PythonIdentifierConverter.Create();
			rustIdConverter = RustIdentifierConverter.Create();
			rustDocWriter = new Documentation.Rust.RustDocCommentWriter(rustIdConverter, ".");

			var dirs = generatorContext.Types.Dirs;
			toFullFileInfo = new Dictionary<TypeId, FullEnumFileInfo?>();
			//TODO: write the remaining ones too
			toFullFileInfo.Add(TypeIds.CC_a, new FullEnumFileInfo(dirs.GetPythonPyFilename("CC_a.py")));
			toFullFileInfo.Add(TypeIds.CC_ae, new FullEnumFileInfo(dirs.GetPythonPyFilename("CC_ae.py")));
			toFullFileInfo.Add(TypeIds.CC_b, new FullEnumFileInfo(dirs.GetPythonPyFilename("CC_b.py")));
			toFullFileInfo.Add(TypeIds.CC_be, new FullEnumFileInfo(dirs.GetPythonPyFilename("CC_be.py")));
			toFullFileInfo.Add(TypeIds.CC_e, new FullEnumFileInfo(dirs.GetPythonPyFilename("CC_e.py")));
			toFullFileInfo.Add(TypeIds.CC_g, new FullEnumFileInfo(dirs.GetPythonPyFilename("CC_g.py")));
			toFullFileInfo.Add(TypeIds.CC_ge, new FullEnumFileInfo(dirs.GetPythonPyFilename("CC_ge.py")));
			toFullFileInfo.Add(TypeIds.CC_l, new FullEnumFileInfo(dirs.GetPythonPyFilename("CC_l.py")));
			toFullFileInfo.Add(TypeIds.CC_le, new FullEnumFileInfo(dirs.GetPythonPyFilename("CC_le.py")));
			toFullFileInfo.Add(TypeIds.CC_ne, new FullEnumFileInfo(dirs.GetPythonPyFilename("CC_ne.py")));
			toFullFileInfo.Add(TypeIds.CC_np, new FullEnumFileInfo(dirs.GetPythonPyFilename("CC_np.py")));
			toFullFileInfo.Add(TypeIds.CC_p, new FullEnumFileInfo(dirs.GetPythonPyFilename("CC_p.py")));
			toFullFileInfo.Add(TypeIds.Code, new FullEnumFileInfo(dirs.GetPythonPyFilename("Code.py")));
			toFullFileInfo.Add(TypeIds.CodeSize, new FullEnumFileInfo(dirs.GetPythonPyFilename("CodeSize.py")));
			toFullFileInfo.Add(TypeIds.ConditionCode, new FullEnumFileInfo(dirs.GetPythonPyFilename("ConditionCode.py")));
			toFullFileInfo.Add(TypeIds.CpuidFeature, new FullEnumFileInfo(dirs.GetPythonPyFilename("CpuidFeature.py")));
			toFullFileInfo.Add(TypeIds.DecoderError, new FullEnumFileInfo(dirs.GetPythonPyFilename("DecoderError.py")));
			toFullFileInfo.Add(TypeIds.DecoderOptions, new FullEnumFileInfo(dirs.GetPythonPyFilename("DecoderOptions.py")));
			toFullFileInfo.Add(TypeIds.EncodingKind, new FullEnumFileInfo(dirs.GetPythonPyFilename("EncodingKind.py")));
			toFullFileInfo.Add(TypeIds.FlowControl, new FullEnumFileInfo(dirs.GetPythonPyFilename("FlowControl.py")));
			toFullFileInfo.Add(TypeIds.FormatMnemonicOptions, new FullEnumFileInfo(dirs.GetPythonPyFilename("FormatMnemonicOptions.py")));
			toFullFileInfo.Add(TypeIds.MandatoryPrefix, new FullEnumFileInfo(dirs.GetPythonPyFilename("MandatoryPrefix.py")));
			toFullFileInfo.Add(TypeIds.MemorySize, new FullEnumFileInfo(dirs.GetPythonPyFilename("MemorySize.py")));
			toFullFileInfo.Add(TypeIds.MemorySizeOptions, new FullEnumFileInfo(dirs.GetPythonPyFilename("MemorySizeOptions.py")));
			toFullFileInfo.Add(TypeIds.Mnemonic, new FullEnumFileInfo(dirs.GetPythonPyFilename("Mnemonic.py")));
			toFullFileInfo.Add(TypeIds.OpAccess, new FullEnumFileInfo(dirs.GetPythonPyFilename("OpAccess.py")));
			toFullFileInfo.Add(TypeIds.OpCodeOperandKind, new FullEnumFileInfo(dirs.GetPythonPyFilename("OpCodeOperandKind.py")));
			toFullFileInfo.Add(TypeIds.OpCodeTableKind, new FullEnumFileInfo(dirs.GetPythonPyFilename("OpCodeTableKind.py")));
			toFullFileInfo.Add(TypeIds.OpKind, new FullEnumFileInfo(dirs.GetPythonPyFilename("OpKind.py")));
			toFullFileInfo.Add(TypeIds.Register, new FullEnumFileInfo(dirs.GetPythonPyFilename("Register.py")));
			//toFullFileInfo.Add(TypeIds.RepPrefixKind, new FullEnumFileInfo(dirs.GetPythonPyFilename("RepPrefixKind.py")));
			toFullFileInfo.Add(TypeIds.RflagsBits, new FullEnumFileInfo(dirs.GetPythonPyFilename("RflagsBits.py")));
			toFullFileInfo.Add(TypeIds.RoundingControl, new FullEnumFileInfo(dirs.GetPythonPyFilename("RoundingControl.py")));
			toFullFileInfo.Add(TypeIds.TupleType, new FullEnumFileInfo(dirs.GetPythonPyFilename("TupleType.py")));
			toFullFileInfo.Add(TypeIds.FormatterSyntax, new FullEnumFileInfo(dirs.GetPythonPyFilename("FormatterSyntax.py")));

			toPartialFileInfo = new Dictionary<TypeId, PartialEnumFileInfo?>();
			toPartialFileInfo.Add(TypeIds.FormatterSyntax, new PartialEnumFileInfo("FormatterSyntax", TargetLanguage.Rust, dirs.GetPythonRustFilename("formatter.rs")));
		}

		public override void Generate(EnumType enumType) {
			if (toFullFileInfo.TryGetValue(enumType.TypeId, out var fullInfo)) {
				if (fullInfo is not null)
					WriteFile(fullInfo, enumType);
			}
			// An enum could be present in both dicts so this should be 'if' and not 'else if'
			if (toPartialFileInfo.TryGetValue(enumType.TypeId, out var partialInfo)) {
				if (partialInfo is not null)
					new FileUpdater(partialInfo.Language, partialInfo.Id, partialInfo.Filename).Generate(writer => WriteEnum(writer, partialInfo, enumType));
			}
		}

		void WriteEnum(FileWriter writer, PartialEnumFileInfo info, EnumType enumType) {
			switch (info.Language) {
			case TargetLanguage.Rust:
				WriteEnumRust(writer, info, enumType);
				break;
			default:
				throw new InvalidOperationException();
			}
		}

		void WriteEnumRust(FileWriter writer, PartialEnumFileInfo info, EnumType enumType) {
			rustDocWriter.WriteSummary(writer, enumType.Documentation, enumType.RawName);
			var enumTypeName = enumType.Name(rustIdConverter);
			foreach (var attr in info.Attributes)
				writer.WriteLine(attr);
			writer.WriteLine(RustConstants.AttributeAllowDeadCode);
			writer.WriteLine($"pub(crate) enum {enumTypeName} {{");
			using (writer.Indent()) {
				uint expectedValue = 0;
				foreach (var value in enumType.Values) {
					if (value.DeprecatedInfo.IsDeprecated)
						continue;
					rustDocWriter.WriteSummary(writer, value.Documentation, enumType.RawName);
					if (enumType.IsFlags)
						writer.WriteLine($"{value.Name(rustIdConverter)} = {NumberFormatter.FormatHexUInt32WithSep(value.Value)},");
					else if (expectedValue != value.Value || enumType.IsPublic)
						writer.WriteLine($"{value.Name(rustIdConverter)} = {value.Value},");
					else
						writer.WriteLine($"{value.Name(rustIdConverter)},");
					expectedValue = value.Value + 1;
				}
			}
			writer.WriteLine("}");
		}

		void WriteFile(FullEnumFileInfo info, EnumType enumType) {
			var docWriter = new PythonDocCommentWriter(pythonIdConverter, isInRootModule: false, ".");
			using (var writer = new FileWriter(TargetLanguage.Python, FileUtils.OpenWrite(info.Filename))) {
				writer.WriteFileHeader();
				writer.WriteLine("# pylint: disable=invalid-name");
				writer.WriteLine("# pylint: disable=line-too-long");
				writer.WriteLine("# pylint: disable=too-many-lines");
				writer.WriteLine();
				docWriter.WriteSummary(writer, enumType.Documentation, enumType.RawName);
				writer.WriteLine();
				writer.WriteLine("from typing import List");
				writer.WriteLine();

				WriteEnumCore(writer, enumType, docWriter);

				writer.WriteLine();
				writer.WriteLine(@"__all__: List[str] = []");
			}
		}

		void WriteEnumCore(FileWriter writer, EnumType enumType, PythonDocCommentWriter docWriter) {
			bool mustHaveDocs = enumType.TypeId != TypeIds.Register && enumType.TypeId != TypeIds.Mnemonic;
			bool uppercaseRawName = PythonUtils.UppercaseEnum(enumType.TypeId.Id1);
			var firstVersion = new Version(1, 9, 1);
			// *****************************************************************************
			// For PERF reasons, we do NOT use Enums. They're incredibly slow to load!
			// Eg. loading 'class Code(IntEnum)' (plus other non-Mnemonic enums and some random
			// code) took ~850ms and when I converted them to constants, it took ~43ms!
			// *****************************************************************************
			foreach (var value in enumType.Values) {
				if (value.DeprecatedInfo.IsDeprecated && value.DeprecatedInfo.Version < firstVersion)
					continue;

				var docs = value.Documentation;
				// Sphinx doesn't include the public enum items (global vars in a module) if they're not documented
				if (string.IsNullOrEmpty(docs)) {
					if (mustHaveDocs)
						throw new InvalidOperationException();
					docs = "<no docs>";
				}

				var numStr = enumType.IsFlags ? NumberFormatter.FormatHexUInt32WithSep(value.Value) : value.Value.ToString();
				string valueName;
				if (uppercaseRawName)
					valueName = value.RawName.ToUpperInvariant();
				else
					valueName = value.Name(pythonIdConverter);
				writer.WriteLine($"{valueName}: int = {numStr}");
				if (value.DeprecatedInfo.IsDeprecated)
					docs = $"DEPRECATED({value.DeprecatedInfo.VersionStr}): {docs}";
				docWriter.WriteSummary(writer, docs, enumType.RawName);
			}
		}
	}
}
