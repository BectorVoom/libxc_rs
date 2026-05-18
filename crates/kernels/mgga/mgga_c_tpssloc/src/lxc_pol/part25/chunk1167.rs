//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1167/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1167<F: Float>(t213: F, t225: F, t852: F, t22986: F, t23272: F, t10103: F, t1880: F, t6553: F, t6571: F, t6552: F, t6554: F, t9516: F) -> (F, F, F) {
    let t82159 = t213 * t852 * t225;
    let t82161 = t22986 * t82159 * t23272;
    let t82165 = t1880 * t6553 * t6571 * t10103;
    let t82169 = t6552 * t6553 * t6554 * t9516;
    (t82161, t82165, t82169)
}
