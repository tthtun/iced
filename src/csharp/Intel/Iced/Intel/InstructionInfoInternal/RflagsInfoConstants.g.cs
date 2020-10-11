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

// ⚠️This file was generated by GENERATOR!🦹‍♂️

#nullable enable

#if INSTR_INFO
namespace Iced.Intel.InstructionInfoInternal {
	static class RflagsInfoConstants {
		public static readonly ushort[] flagsRead = new ushort[63] {
			0x0000,// None
			0x0000,// C_A
			0x0000,// C_acos_S_pz
			0x0000,// C_c
			0x0000,// C_cos_S_pz_U_a
			0x0000,// C_d
			0x0000,// C_i
			0x0000,// C_u
			0x0008,// R_a_W_ac_U_opsz
			0x0018,// R_ac_W_acpsz_U_o
			0x00FF,// R_acopszid
			0x01FF,// R_acopszidA
			0x01FF,// R_acopszidA_W_acopszidA
			0x003E,// R_acpsz
			0x0010,// R_c
			0x0010,// R_c_W_acopsz
			0x0010,// R_c_W_c
			0x0010,// R_c_W_c_U_o
			0x0010,// R_c_W_co
			0x0014,// R_cz
			0x0040,// R_d
			0x0040,// R_d_W_acopsz
			0x0001,// R_o
			0x0001,// R_o_W_o
			0x0003,// R_os
			0x0007,// R_osz
			0x0020,// R_p
			0x0002,// R_s
			0x0200,// R_u_W_c_C_aopsz
			0x0004,// R_z
			0x0000,// S_A
			0x0000,// S_c
			0x0000,// S_d
			0x0000,// S_i
			0x0000,// S_u
			0x0000,// U_acopsz
			0x0000,// W_acopsz
			0x0000,// W_acopszdA_S_u
			0x0000,// W_acopszid
			0x0000,// W_acopszidA
			0x0000,// W_acpsz
			0x0000,// W_aopsz
			0x0000,// W_c
			0x0000,// W_c_C_aopsz
			0x0000,// W_c_U_aops
			0x0000,// W_c_U_o
			0x0000,// W_co
			0x0000,// W_co_U_apsz
			0x0000,// W_copsz_U_a
			0x0000,// W_cosz_C_ap
			0x0000,// W_cpsz_U_ao
			0x0000,// W_cpz_C_aos
			0x0000,// W_cs_C_oz_U_ap
			0x0000,// W_csz_C_o_U_ap
			0x0000,// W_cz_C_aops
			0x0000,// W_cz_U_aops
			0x0000,// W_psz_C_co_U_a
			0x0000,// W_psz_U_aco
			0x0000,// W_sz_C_co_U_ap
			0x0000,// W_z
			0x0000,// W_z_C_acops
			0x0000,// W_z_C_co_U_aps
			0x0000,// W_z_U_acops
		};
		public static readonly ushort[] flagsUndefined = new ushort[63] {
			0x0000,// None
			0x0000,// C_A
			0x0000,// C_acos_S_pz
			0x0000,// C_c
			0x0008,// C_cos_S_pz_U_a
			0x0000,// C_d
			0x0000,// C_i
			0x0000,// C_u
			0x0027,// R_a_W_ac_U_opsz
			0x0001,// R_ac_W_acpsz_U_o
			0x0000,// R_acopszid
			0x0000,// R_acopszidA
			0x0000,// R_acopszidA_W_acopszidA
			0x0000,// R_acpsz
			0x0000,// R_c
			0x0000,// R_c_W_acopsz
			0x0000,// R_c_W_c
			0x0001,// R_c_W_c_U_o
			0x0000,// R_c_W_co
			0x0000,// R_cz
			0x0000,// R_d
			0x0000,// R_d_W_acopsz
			0x0000,// R_o
			0x0000,// R_o_W_o
			0x0000,// R_os
			0x0000,// R_osz
			0x0000,// R_p
			0x0000,// R_s
			0x0000,// R_u_W_c_C_aopsz
			0x0000,// R_z
			0x0000,// S_A
			0x0000,// S_c
			0x0000,// S_d
			0x0000,// S_i
			0x0000,// S_u
			0x003F,// U_acopsz
			0x0000,// W_acopsz
			0x0000,// W_acopszdA_S_u
			0x0000,// W_acopszid
			0x0000,// W_acopszidA
			0x0000,// W_acpsz
			0x0000,// W_aopsz
			0x0000,// W_c
			0x0000,// W_c_C_aopsz
			0x002B,// W_c_U_aops
			0x0001,// W_c_U_o
			0x0000,// W_co
			0x002E,// W_co_U_apsz
			0x0008,// W_copsz_U_a
			0x0000,// W_cosz_C_ap
			0x0009,// W_cpsz_U_ao
			0x0000,// W_cpz_C_aos
			0x0028,// W_cs_C_oz_U_ap
			0x0028,// W_csz_C_o_U_ap
			0x0000,// W_cz_C_aops
			0x002B,// W_cz_U_aops
			0x0008,// W_psz_C_co_U_a
			0x0019,// W_psz_U_aco
			0x0028,// W_sz_C_co_U_ap
			0x0000,// W_z
			0x0000,// W_z_C_acops
			0x002A,// W_z_C_co_U_aps
			0x003B,// W_z_U_acops
		};
		public static readonly ushort[] flagsWritten = new ushort[63] {
			0x0000,// None
			0x0000,// C_A
			0x0000,// C_acos_S_pz
			0x0000,// C_c
			0x0000,// C_cos_S_pz_U_a
			0x0000,// C_d
			0x0000,// C_i
			0x0000,// C_u
			0x0018,// R_a_W_ac_U_opsz
			0x003E,// R_ac_W_acpsz_U_o
			0x0000,// R_acopszid
			0x0000,// R_acopszidA
			0x01FF,// R_acopszidA_W_acopszidA
			0x0000,// R_acpsz
			0x0000,// R_c
			0x003F,// R_c_W_acopsz
			0x0010,// R_c_W_c
			0x0010,// R_c_W_c_U_o
			0x0011,// R_c_W_co
			0x0000,// R_cz
			0x0000,// R_d
			0x003F,// R_d_W_acopsz
			0x0000,// R_o
			0x0001,// R_o_W_o
			0x0000,// R_os
			0x0000,// R_osz
			0x0000,// R_p
			0x0000,// R_s
			0x0010,// R_u_W_c_C_aopsz
			0x0000,// R_z
			0x0000,// S_A
			0x0000,// S_c
			0x0000,// S_d
			0x0000,// S_i
			0x0000,// S_u
			0x0000,// U_acopsz
			0x003F,// W_acopsz
			0x017F,// W_acopszdA_S_u
			0x00FF,// W_acopszid
			0x01FF,// W_acopszidA
			0x003E,// W_acpsz
			0x002F,// W_aopsz
			0x0010,// W_c
			0x0010,// W_c_C_aopsz
			0x0010,// W_c_U_aops
			0x0010,// W_c_U_o
			0x0011,// W_co
			0x0011,// W_co_U_apsz
			0x0037,// W_copsz_U_a
			0x0017,// W_cosz_C_ap
			0x0036,// W_cpsz_U_ao
			0x0034,// W_cpz_C_aos
			0x0012,// W_cs_C_oz_U_ap
			0x0016,// W_csz_C_o_U_ap
			0x0014,// W_cz_C_aops
			0x0014,// W_cz_U_aops
			0x0026,// W_psz_C_co_U_a
			0x0026,// W_psz_U_aco
			0x0006,// W_sz_C_co_U_ap
			0x0004,// W_z
			0x0004,// W_z_C_acops
			0x0004,// W_z_C_co_U_aps
			0x0004,// W_z_U_acops
		};
		public static readonly ushort[] flagsCleared = new ushort[63] {
			0x0000,// None
			0x0100,// C_A
			0x001B,// C_acos_S_pz
			0x0010,// C_c
			0x0013,// C_cos_S_pz_U_a
			0x0040,// C_d
			0x0080,// C_i
			0x0200,// C_u
			0x0000,// R_a_W_ac_U_opsz
			0x0000,// R_ac_W_acpsz_U_o
			0x0000,// R_acopszid
			0x0000,// R_acopszidA
			0x0000,// R_acopszidA_W_acopszidA
			0x0000,// R_acpsz
			0x0000,// R_c
			0x0000,// R_c_W_acopsz
			0x0000,// R_c_W_c
			0x0000,// R_c_W_c_U_o
			0x0000,// R_c_W_co
			0x0000,// R_cz
			0x0000,// R_d
			0x0000,// R_d_W_acopsz
			0x0000,// R_o
			0x0000,// R_o_W_o
			0x0000,// R_os
			0x0000,// R_osz
			0x0000,// R_p
			0x0000,// R_s
			0x002F,// R_u_W_c_C_aopsz
			0x0000,// R_z
			0x0000,// S_A
			0x0000,// S_c
			0x0000,// S_d
			0x0000,// S_i
			0x0000,// S_u
			0x0000,// U_acopsz
			0x0000,// W_acopsz
			0x0000,// W_acopszdA_S_u
			0x0000,// W_acopszid
			0x0000,// W_acopszidA
			0x0000,// W_acpsz
			0x0000,// W_aopsz
			0x0000,// W_c
			0x002F,// W_c_C_aopsz
			0x0000,// W_c_U_aops
			0x0000,// W_c_U_o
			0x0000,// W_co
			0x0000,// W_co_U_apsz
			0x0000,// W_copsz_U_a
			0x0028,// W_cosz_C_ap
			0x0000,// W_cpsz_U_ao
			0x000B,// W_cpz_C_aos
			0x0005,// W_cs_C_oz_U_ap
			0x0001,// W_csz_C_o_U_ap
			0x002B,// W_cz_C_aops
			0x0000,// W_cz_U_aops
			0x0011,// W_psz_C_co_U_a
			0x0000,// W_psz_U_aco
			0x0011,// W_sz_C_co_U_ap
			0x0000,// W_z
			0x003B,// W_z_C_acops
			0x0011,// W_z_C_co_U_aps
			0x0000,// W_z_U_acops
		};
		public static readonly ushort[] flagsSet = new ushort[63] {
			0x0000,// None
			0x0000,// C_A
			0x0024,// C_acos_S_pz
			0x0000,// C_c
			0x0024,// C_cos_S_pz_U_a
			0x0000,// C_d
			0x0000,// C_i
			0x0000,// C_u
			0x0000,// R_a_W_ac_U_opsz
			0x0000,// R_ac_W_acpsz_U_o
			0x0000,// R_acopszid
			0x0000,// R_acopszidA
			0x0000,// R_acopszidA_W_acopszidA
			0x0000,// R_acpsz
			0x0000,// R_c
			0x0000,// R_c_W_acopsz
			0x0000,// R_c_W_c
			0x0000,// R_c_W_c_U_o
			0x0000,// R_c_W_co
			0x0000,// R_cz
			0x0000,// R_d
			0x0000,// R_d_W_acopsz
			0x0000,// R_o
			0x0000,// R_o_W_o
			0x0000,// R_os
			0x0000,// R_osz
			0x0000,// R_p
			0x0000,// R_s
			0x0000,// R_u_W_c_C_aopsz
			0x0000,// R_z
			0x0100,// S_A
			0x0010,// S_c
			0x0040,// S_d
			0x0080,// S_i
			0x0200,// S_u
			0x0000,// U_acopsz
			0x0000,// W_acopsz
			0x0200,// W_acopszdA_S_u
			0x0000,// W_acopszid
			0x0000,// W_acopszidA
			0x0000,// W_acpsz
			0x0000,// W_aopsz
			0x0000,// W_c
			0x0000,// W_c_C_aopsz
			0x0000,// W_c_U_aops
			0x0000,// W_c_U_o
			0x0000,// W_co
			0x0000,// W_co_U_apsz
			0x0000,// W_copsz_U_a
			0x0000,// W_cosz_C_ap
			0x0000,// W_cpsz_U_ao
			0x0000,// W_cpz_C_aos
			0x0000,// W_cs_C_oz_U_ap
			0x0000,// W_csz_C_o_U_ap
			0x0000,// W_cz_C_aops
			0x0000,// W_cz_U_aops
			0x0000,// W_psz_C_co_U_a
			0x0000,// W_psz_U_aco
			0x0000,// W_sz_C_co_U_ap
			0x0000,// W_z
			0x0000,// W_z_C_acops
			0x0000,// W_z_C_co_U_aps
			0x0000,// W_z_U_acops
		};
		public static readonly ushort[] flagsModified = new ushort[63] {
			0x0000,// None
			0x0100,// C_A
			0x003F,// C_acos_S_pz
			0x0010,// C_c
			0x003F,// C_cos_S_pz_U_a
			0x0040,// C_d
			0x0080,// C_i
			0x0200,// C_u
			0x003F,// R_a_W_ac_U_opsz
			0x003F,// R_ac_W_acpsz_U_o
			0x0000,// R_acopszid
			0x0000,// R_acopszidA
			0x01FF,// R_acopszidA_W_acopszidA
			0x0000,// R_acpsz
			0x0000,// R_c
			0x003F,// R_c_W_acopsz
			0x0010,// R_c_W_c
			0x0011,// R_c_W_c_U_o
			0x0011,// R_c_W_co
			0x0000,// R_cz
			0x0000,// R_d
			0x003F,// R_d_W_acopsz
			0x0000,// R_o
			0x0001,// R_o_W_o
			0x0000,// R_os
			0x0000,// R_osz
			0x0000,// R_p
			0x0000,// R_s
			0x003F,// R_u_W_c_C_aopsz
			0x0000,// R_z
			0x0100,// S_A
			0x0010,// S_c
			0x0040,// S_d
			0x0080,// S_i
			0x0200,// S_u
			0x003F,// U_acopsz
			0x003F,// W_acopsz
			0x037F,// W_acopszdA_S_u
			0x00FF,// W_acopszid
			0x01FF,// W_acopszidA
			0x003E,// W_acpsz
			0x002F,// W_aopsz
			0x0010,// W_c
			0x003F,// W_c_C_aopsz
			0x003B,// W_c_U_aops
			0x0011,// W_c_U_o
			0x0011,// W_co
			0x003F,// W_co_U_apsz
			0x003F,// W_copsz_U_a
			0x003F,// W_cosz_C_ap
			0x003F,// W_cpsz_U_ao
			0x003F,// W_cpz_C_aos
			0x003F,// W_cs_C_oz_U_ap
			0x003F,// W_csz_C_o_U_ap
			0x003F,// W_cz_C_aops
			0x003F,// W_cz_U_aops
			0x003F,// W_psz_C_co_U_a
			0x003F,// W_psz_U_aco
			0x003F,// W_sz_C_co_U_ap
			0x0004,// W_z
			0x003F,// W_z_C_acops
			0x003F,// W_z_C_co_U_aps
			0x003F,// W_z_U_acops
		};
	}
}
#endif
