//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta250 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk992;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk993;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta250<F: Float>(t11516: F, t9288: F, t3440: F, t3441: F, t1177: F, t1178: F, t9258: F, t1176: F, t698: F, t1179: F, t1174: F, t3431: F, t3460: F, t3456: F, t135: F, t3439: F, t3442: F, t11499: F, t11505: F, t11510: F, t11514: F, t3247: F, t405: F) -> (F, F, F, F, F, F, F) {
        let (t11517, t11518, t11521, t11522, t11525, t11526, t11529, t11531, t11533) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk992::<F>(t11516, t9288, t3440, t3441, t1177, t1178, t9258, t1176, t698, t1179, t1174, t3431, t3460);
        let (t11539, t11543) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk993::<F>(t11533, t1174, t3431, t3456, t135, t3439, t3442, t11499, t11505, t11510, t11514, t11518, t11522, t11526, t11531);
        let t11545 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk994::<F>(t3247, t405);
    (t11517, t11521, t11525, t11529, t11539, t11543, t11545)
}
