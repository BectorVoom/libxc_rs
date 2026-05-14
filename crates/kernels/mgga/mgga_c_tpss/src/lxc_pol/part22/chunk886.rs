//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 886/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk886<F: Float>(t9213: F, t1052: F, t2953: F, t412: F, t2956: F, t420: F, t1049: F, t2929: F, t1022: F, t2909: F, t394: F, t2912: F, t407: F, t1023: F, t2854: F, t1019: F, t2910: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9438 = 0.16068111111111111111e1 * t9213;
    let t9464 = 1.0 / t2953 / t1052;
    let t9465 = t412 * t9464;
    let t9467 = 1.0 / t2956 / t420;
    let t9471 = t1049 * t2929;
    let t9477 = 0.53272592592592592592e-1 * t9213;
    let t9492 = 1.0 / t2909 / t1022;
    let t9493 = t394 * t9492;
    let t9495 = 1.0 / t2912 / t407;
    let t9499 = t2854 * t1023;
    let t9504 = t1019 * t2910;
    (t9438, t9465, t9467, t9471, t9477, t9493, t9495, t9499, t9504)
}
