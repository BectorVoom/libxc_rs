//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 994/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk994<F: Float>(t1880: F, t29055: F, t6553: F, t6571: F, t25224: F, t33408: F, t23270: F, t25038: F, t31337: F, t5527: F, t121634: F, t1484: F, t22986: F) -> (F, F, F, F) {
    let t127778 = t1880 * t6553 * t6571 * t29055;
    let t127786 = t1880 * t25224 * t33408;
    let t127790 = t25038 * t23270 * t31337 * t5527;
    let t127794 = t22986 * t23270 * t121634 * t1484;
    (t127778, t127786, t127790, t127794)
}
