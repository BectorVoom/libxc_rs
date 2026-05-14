//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1228/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1228<F: Float>(t21190: F, t485: F, t626: F, t3493: F, t6113: F, t6103: F, t1688: F, t5314: F, t1600: F, t6112: F, t13565: F, t21180: F, t4674: F, t93: F, t6234: F, t1165: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t21191 = t485 * t21190;
    let t21193 = 2.0 * t626 * t21191;
    let t21198 = 4.0 * t3493 * t6113;
    let t21202 = 4.0 * t6103 * t6113;
    let t21203 = t5314 * t1688;
    let t21205 = 2.0 * t626 * t21203;
    let t21211 = t1600 * t6112;
    let t21213 = 4.0 * t626 * t21211;
    let t21222 = 2.0 * t13565 * t1688;
    let t21224 = 4.0 * t21180 * t1688;
    let t21226 = 4.0 * t3493 * t6112;
    let t21227 = t93 * t4674;
    let t21229 = 2.0 * t21227 * t1688;
    let t21231 = 4.0 * t6234 * t6112;
    let t21233 = 2.0 * t1165 * t21190;
    (t21191, t21193, t21198, t21202, t21203, t21205, t21211, t21213, t21222, t21224, t21226, t21227, t21229, t21231, t21233)
}
