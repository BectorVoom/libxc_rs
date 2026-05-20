//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1940;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1941;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1942;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1943;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta421<F: Float>(t1751: F, t3493: F, t1246: F, t3507: F, t3625: F, t1932: F, t475: F, t1755: F, t1720: F, t3030: F, t3609: F, t11877: F, t11881: F, t1244: F, t1249: F, t14986: F, t14989: F, t14992: F, t14997: F, t15001: F, t15004: F, t15009: F, t1729: F, t1756: F, t3604: F, t3610: F, t3613: F, t3617: F, t3624: F, t3628: F, t4964: F, t5064: F, t5073: F, t1009: F, t4940: F, t1243: F, t14701: F, t14833: F, t14835: F, t14837: F, t14840: F, t14844: F, t14847: F, t14849: F, t14852: F, t14857: F, t14860: F, t14862: F, t14864: F, t14866: F, t14916: F, t14936: F, t14939: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15016, t15018, t15019, t15022, t15023, t15026, t15027) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1940::<F>(t1751, t3493, t1246, t3507, t3625, t1932, t475, t1755, t1720, t3030, t3609);
        let t15030 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1941::<F>(t11877, t11881, t1244, t1249, t14986, t14989, t14992, t14997, t15001, t15004, t15009, t15016, t15019, t15023, t15027, t1729, t1756, t3604, t3610, t3613, t3617, t3624, t3628, t4964, t5064, t5073);
        let (t15031, t15032) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1942::<F>(t1009, t4940, t1243);
        let t15035 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1943::<F>(t14701, t14833, t14835, t14837, t14840, t14844, t14847, t14849, t14852, t14857, t14860, t14862, t14864, t14866, t14916, t14936, t14939);
    (t15016, t15018, t15019, t15022, t15023, t15026, t15027, t15030, t15031, t15032, t15035)
}
