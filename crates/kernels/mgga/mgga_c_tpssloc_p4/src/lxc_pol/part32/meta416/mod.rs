//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta416 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1607;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1608;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1609;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1610;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1611;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1612;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1613;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta416<F: Float>(t11204: F, t11211: F, t14702: F, t14868: F, t14870: F, t18742: F, t18747: F, t18749: F, t18752: F, t18755: F, t18757: F, t11137: F, t14818: F, t18227: F, t18239: F, t18497: F, t18500: F, t18503: F, t18508: F, t18510: F, t18515: F, t18518: F, t11195: F, t14720: F, t14766: F, t14886: F, t14890: F, t18203: F, t18208: F, t18213: F, t18217: F, t18219: F, t18223: F, t18229: F, t18234: F, t18243: F, t18494: F, t18505: F, t18512: F, t18521: F, t18731: F, t18762: F, t1118: F, t1099: F, t11185: F, t6024: F, t1128: F, t6031: F, t11317: F, t15072: F, t15074: F, t11314: F, t14722: F, t15083: F, t15094: F, t1137: F, t1147: F, t6063: F, t1129: F, t11303: F, t11361: F, t1138: F, t11415: F, t1157: F, t15121: F, t15141: F, t1683: F, t1695: F, t3327: F, t4797: F, t4820: F, t4835: F, t4858: F, t6037: F, t6053: F, t6056: F, t6088: F, t18630: F, t18673: F, t18789: F, t300: F, t3400: F, t6084: F, t4883: F, t1164: F, t18247: F, t18249: F, t18251: F, t18257: F, t18261: F, t18264: F, t18268: F, t18270: F, t18273: F, t18278: F, t18282: F, t18285: F, t18672: F, t18676: F, t18679: F, t1166: F, t4874: F, t3411: F, t6098: F, t4869: F, t4884: F, t1156: F, t18785: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18810, t18832) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1607::<F>(t11204, t11211, t14702, t14868, t14870, t18742, t18747, t18749, t18752, t18755, t18757, t11137, t14818, t18227, t18239, t18497, t18500, t18503, t18508, t18510, t18515, t18518);
        let t18834 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1608::<F>(t11195, t14720, t14766, t14886, t14890, t18203, t18208, t18213, t18217, t18219, t18223, t18229, t18234, t18243, t18494, t18505, t18512, t18521, t18731, t18762, t18810, t18832);
        let (t18837, t18839, t18840, t18869) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1609::<F>(t1118, t18834, t1099, t11185, t6024, t1128, t6031, t11211, t11317, t14702, t15072, t15074, t18742, t18747, t18749, t18752, t18755, t18757);
        let t18893 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1610::<F>(t11137, t14818, t18227, t18239, t18497, t18500, t18503, t18508, t18510, t18515, t18518, t11314, t14722, t14766, t15083, t15094, t18203, t18208, t18213, t18217, t18219, t18223, t18229, t18234, t18243, t18494, t18505, t18512, t18521, t18731, t18762, t18869);
        let t18906 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1611::<F>(t1137, t18893, t1147, t6063, t1129, t11303, t11361, t1138, t11415, t1157, t15121, t15141, t1683, t1695, t18837, t18839, t18840, t3327, t4797, t4820, t4835, t4858, t6037, t6053, t6056, t6088);
        let (t18909, t18913, t18914) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1612::<F>(t18630, t18673, t18789, t18906, t300, t3400, t6084, t4883, t1164, t18247, t18249, t18251, t18257, t18261, t18264, t18268, t18270, t18273, t18278, t18282, t18285, t18672, t18676, t18679);
        let (t18917, t18920, t18922, t18924, t18926) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1613::<F>(t300, t6063, t1166, t4858, t4874, t1164, t3411, t6098, t4869, t4884, t1147, t1156, t18785);
    (t18837, t18839, t18909, t18913, t18914, t18917, t18920, t18922, t18924, t18926)
}
