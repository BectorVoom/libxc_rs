//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1919/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1919<F: Float>(t23270: F, t28267: F, t22986: F, t225: F, t258: F, t5631: F, t214: F, t1880: F, t5544: F, t6554: F, t6553: F, t6552: F) -> (F, F, F, F, F, F, F, F) {
    let t28268 = t23270 * t28267;
    let t28269 = t22986 * t28268;
    let t28272 = t5631 * t225 * t258;
    let t28273 = t214 * t28272;
    let t28274 = t1880 * t28273;
    let t28276 = t6554 * t5544;
    let t28277 = t6553 * t28276;
    let t28278 = t6552 * t28277;
    (t28268, t28269, t28272, t28273, t28274, t28276, t28277, t28278)
}
