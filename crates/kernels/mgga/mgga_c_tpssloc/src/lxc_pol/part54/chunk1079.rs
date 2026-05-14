//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1079/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1079<F: Float>(t2075: F, t2114: F, t31305: F, t31306: F, t31519: F, t31521: F, t31523: F, t31525: F, t31527: F, t31531: F, t31539: F, t31542: F, t31544: F, t31548: F, t7156: F, t7264: F) -> (F,) {
    let t32371 = -t2075 * t7264 - t2114 * t7156 + t31305 + t31306 - t31519 - t31521 - t31523 - t31525 - t31527 - t31531 - t31539 - t31542 - t31544 - t31548;
    (t32371,)
}
