//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1102/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1102<F: Float>(t11910: F, t11942: F, t11932: F, t11938: F, t11952: F, t11955: F, t11960: F, t11963: F, t9221: F, t9223: F, t9226: F, t9228: F) -> (F, F) {
    let t12115 = F::new(0.22076e0) * t11910;
    let t12129 = F::new(0.20128333333333333334e0) * t11942;
    let t12133 = F::new(0.26837777777777777778e0) * t9221 + F::new(0.67094444444444444447e-1) * t9223 - F::new(0.20128333333333333334e0) * t9226 - F::new(0.10064166666666666667e0) * t9228 + F::new(0.36793333333333333334e-1) * t11932 + F::new(0.258925e1) * t11955 + F::new(0.13418888888888888889e0) * t11938 - t12129 + F::new(0.301925e0) * t11952 + F::new(0.16504875e0) * t11960 + F::new(0.36793333333333333333e-1) * t11963;
    (t12115, t12133)
}
