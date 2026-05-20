//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1865;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta580<F: Float>(t13184: F, t221: F, t87420: F, t25120: F, t6604: F, t81962: F, t13196: F, t25119: F, t841: F, t13204: F, t6581: F, t7500: F, t81911: F, t22690: F, t23122: F, t4119: F, t25064: F, t81902: F, t23077: F, t6646: F, t23098: F, t7496: F, t6590: F, t25130: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t87422, t87425, t87428, t87430, t87432) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1865::<F>(t13184, t221, t87420, t25120, t6604, t81962, t13196, t25119, t841, t13204, t6581, t7500, t81911);
        let (t87443, t87445, t87449, t87453) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1866::<F>(t22690, t23122, t4119, t841, t25064, t81902, t23077, t6646, t23098, t7496, t6590, t25130);
    (t87422, t87425, t87428, t87430, t87432, t87443, t87445, t87449, t87453)
}
