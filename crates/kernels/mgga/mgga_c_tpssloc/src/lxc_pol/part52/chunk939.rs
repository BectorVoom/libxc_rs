//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 939/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk939<F: Float>(t6624: F, t814: F, t2627: F, t6604: F, t6579: F, t6649: F, t1879: F, t22715: F) -> (F, F, F, F) {
    let t22992 = t814 * t6624;
    let t22996 = t6604 * t2627;
    let t23002 = t6579 * t6649;
    let t23012 = t22715 * t1879;
    (t22992, t22996, t23002, t23012)
}
