//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 665/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk665<F: Float>(t3: F, t7222: F, t112: F, t2098: F, t2039: F, t671: F, t1401: F, t3938: F, t3941: F, t577: F, t7056: F, t590: F, t60: F, t192: F, t533: F, t1390: F, t2094: F) -> (F, F, F, F, F, F, F) {
    let t7223 = t3 * t7222;
    let t7230 = t2098 * t112;
    let t7235 = t2039 * t671;
    let t7240 = 0.45e1 * t7222 * t577 + 0.135e2 * t7230 * t671 + 0.135e2 * t3938 * t2039 + 27.0 * t3941 * t7235 + 0.135e2 * t1401 * t7056;
    let t8705 = 1.0 / t60 / t590;
    let t8944 = t192 * t533;
    let t9016 = t2094 * t1390;
    (t7223, t7230, t7235, t7240, t8705, t8944, t9016)
}
