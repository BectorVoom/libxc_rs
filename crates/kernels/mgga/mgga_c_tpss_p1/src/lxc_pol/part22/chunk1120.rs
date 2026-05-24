//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1120/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1120<F: Float>(t1125: F, t12367: F, t3062: F, t4258: F, t1114: F, t581: F, t4051: F, t3068: F, t1113: F, t1561: F, t1014: F, t450: F) -> (F, F, F, F, F, F) {
    let t12368 = t1125 * t12367;
    let t12371 = t4258 * t3062 / F::new(432.0);
    let t12372 = t1114 * t581;
    let t12373 = t4051 * t12372;
    let t12374 = t3068 * t12373;
    let t12377 = t1561 * t1113;
    let t12378 = t450 * t1014;
    (t12368, t12371, t12372, t12374, t12377, t12378)
}
