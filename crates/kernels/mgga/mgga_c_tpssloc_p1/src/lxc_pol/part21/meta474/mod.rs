//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2057;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2058;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2059;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta474<F: Float>(t119: F, t16018: F, t210: F, t12308: F, t12310: F, t12317: F, t12323: F, t12325: F, t12330: F, t12336: F, t1315: F, t1363: F, t1369: F, t16321: F, t16325: F, t16331: F, t16333: F, t16338: F, t16341: F, t16346: F, t16347: F, t16350: F, t16354: F, t1831: F, t3783: F, t3876: F, t5240: F, t5314: F, t559: F, t120: F, t5187: F, t1352: F, t3805: F, t3851: F, t5301: F, t1810: F, t3734: F, t3856: F, t3793: F, t5248: F, t5249: F, t3802: F, t5234: F, t3788: F, t836: F, t1336: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16356, t16361) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2057::<F>(t119, t16018, t210, t12308, t12310, t12317, t12323, t12325, t12330, t12336, t1315, t1363, t1369, t16321, t16325, t16331, t16333, t16338, t16341, t16346, t16347, t16350, t16354, t1831, t3783, t3876, t5240, t5314, t559);
        let (t16366, t16370, t16379, t16383, t16387) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2058::<F>(t120, t5187, t1352, t3805, t3851, t5301, t1810, t210, t3734, t3856, t3793, t5248, t5249);
        let (t16391, t16394) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2059::<F>(t3793, t3805, t5301, t3802, t5234);
        let (t16397, t16398) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2060::<F>(t3788, t836, t1336);
    (t16356, t16361, t16366, t16370, t16379, t16383, t16387, t16391, t16394, t16397, t16398)
}
