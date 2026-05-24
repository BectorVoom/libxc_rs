//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 912/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk912<F: Float>(t2162: F, t2364: F, t219: F, t2399: F, t810: F, t73: F, t2398: F, t768: F, t242: F, t2675: F, t2704: F, t946: F) -> (F, F, F, F, F, F, F) {
    let t8330 = t2162 * t2364;
    let t8339 = t2399 * t219;
    let t8346 = t810 * t810;
    let t8347 = F::new(1.0) / t8346;
    let t8348 = t73 * t8347;
    let t8372 = t768 * t2398;
    let t8430 = t242 * t2675 * t2704;
    let t8431 = t946 * t8430;
    (t8330, t8339, t8346, t8347, t8348, t8372, t8431)
}
