//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta848 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3070;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3071;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta848<F: Float>(t4745: F, t51246: F, t14838: F, t15051: F, t15054: F, t15057: F, t51249: F, t4786: F, t51402: F, t14850: F, t15061: F, t15064: F, t15068: F, t51120: F, t11185: F, t18677: F, t1098: F, t18245: F, t1119: F, t18686: F, t3308: F, t3312: F, t5983: F, t3316: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t63731, t63733, t63735, t63737, t63739, t63741, t63743) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3070::<F>(t4745, t51246, t14838, t15051, t15054, t15057, t51249, t4786, t51402, t14850, t15061, t15064);
        let (t63745, t63747, t63752, t63754, t63757) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3071::<F>(t15068, t51120, t11185, t18677, t1098, t18245, t1119, t18686, t3308, t3312, t5983, t3316);
    (t63731, t63733, t63735, t63737, t63739, t63741, t63743, t63745, t63747, t63752, t63754, t63757)
}
