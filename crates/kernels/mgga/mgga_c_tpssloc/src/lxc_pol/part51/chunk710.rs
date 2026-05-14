//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 710/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk710<F: Float>(t6635: F, t6644: F, t2047: F, t814: F, t829: F, t235: F, t7084: F, t2051: F, t226: F, t6641: F, t6650: F, t6654: F, t808: F, t812: F) -> (F, F, F, F, F, F) {
    let t7095 = 0.38381794893125283518e-1 * t6635;
    let t7097 = 0.82246703342411321825e-2 * t6644;
    let t7101 = t814 * t2047;
    let t7102 = t7101 * t829;
    let t7104 = t235 * t7084;
    let t7106 = -t7095 - 0.3289868133696452873e-1 * t6641 - t7097 - 0.16449340668482264365e-1 * t6650 + 0.16449340668482264365e-1 * t6654 + t808 * t2051 - t812 * t7102 + t226 * t7104;
    (t7095, t7097, t7101, t7102, t7104, t7106)
}
