//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1098/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1098<F: Float>(t11910: F, t11942: F, t11932: F, t11938: F, t11952: F, t11955: F, t11960: F, t11963: F, t9221: F, t9223: F, t9226: F, t9228: F) -> (F, F) {
    let t12046 = F::new(0.27785333333333333334e0) * t11910;
    let t12060 = F::new(0.34431666666666666666e0) * t11942;
    let t12064 = F::new(0.45908888888888888888e0) * t9221 + F::new(0.11477222222222222222e0) * t9223 - F::new(0.34431666666666666666e0) * t9226 - F::new(0.17215833333333333333e0) * t9228 + F::new(0.46308888888888888889e-1) * t11932 + F::new(0.3529725e1) * t11955 + F::new(0.22954444444444444444e0) * t11938 - t12060 + F::new(0.516475e0) * t11952 + F::new(0.6311625e0) * t11960 + F::new(0.46308888888888888889e-1) * t11963;
    (t12046, t12064)
}
