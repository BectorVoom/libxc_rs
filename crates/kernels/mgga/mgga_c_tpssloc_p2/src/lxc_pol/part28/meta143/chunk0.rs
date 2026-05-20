//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 755/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk755<F: Float>(t3106: F, t3165: F, t349: F, t1050: F, t225: F, t1053: F, t386: F, t68: F, t1065: F, t1057: F, t3112: F) -> (F, F, F, F, F, F, F) {
    let t3166 = t3106 + t3165;
    let t3167 = t349 * t3166;
    let t3169 = t1050 * t225;
    let t3173 = F::new(1.0) / t1053 / t386;
    let t3174 = t68 * t3173;
    let t3175 = t1065 * t1065;
    let t3176 = t3174 * t3175;
    let t3180 = t3112 * t1057;
    (t3166, t3167, t3169, t3174, t3175, t3176, t3180)
}
