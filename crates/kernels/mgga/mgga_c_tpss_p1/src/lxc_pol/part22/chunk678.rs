//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 678/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk678<F: Float>(t2140: F, t512: F, t1212: F, t756: F, t1215: F, t159: F, t527: F, t210: F, t1206: F) -> (F, F, F, F, F, F) {
    let t3239 = F::new(35.0) / F::new(432.0) * t2140 * t512;
    let t3240 = t756 * t1212;
    let t3241 = t3240 * t1215;
    let t3243 = t159 * t527;
    let t3244 = t210 * t3243;
    let t3245 = t1206 * t1206;
    (t3239, t3240, t3241, t3243, t3244, t3245)
}
