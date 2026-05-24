//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 622/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk622<F: Float>(t289: F, t8188: F, t2232: F, t275: F, t2231: F, t302: F, t72: F, t1347: F, t703: F, t1288: F, t702: F, t7897: F) -> (F, F, F, F, F, F, F, F) {
    let t8189 = t289 * t8188;
    let t8190 = F::new(0.4726e1) * t8189;
    let t8191 = t275 * t2232;
    let t8198 = t302 * t2231;
    let t8199 = t72 * t8198;
    let t8200 = F::new(2.0) * t8199;
    let t8201 = t1347 * t703;
    let t8202 = t1288 * t702;
    let t8203 = t72 * t8202;
    let t8204 = F::cast_from(0.2993560425465952141e-1_f64) * t7897;
    (t8190, t8191, t8198, t8200, t8201, t8202, t8203, t8204)
}
