//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1325/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1325<F: Float>(t1206: F, t19581: F, t1338: F, t2053: F, t3537: F, t623: F, t2049: F, t6076: F, t77: F, t1317: F, t5506: F, t19407: F, t619: F) -> (F, F, F, F, F, F) {
    let t65085 = t19581 * t1206;
    let t65094 = t2053 * t1338;
    let t65097 = t623 * t3537;
    let t65152 = t77 * t6076 * t2049;
    let t65157 = t5506 * t1317;
    let t65162 = t77 * t19407 * t619;
    (t65085, t65094, t65097, t65152, t65157, t65162)
}
