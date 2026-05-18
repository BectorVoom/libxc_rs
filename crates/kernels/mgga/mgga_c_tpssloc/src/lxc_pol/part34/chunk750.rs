//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 750/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk750<F: Float>(t11203: F, t241: F, t3439: F, t407: F, t11135: F, t410: F, t417: F, t1097: F, t3311: F, t409: F, t3314: F, t422: F) -> (F, F, F, F, F, F, F) {
    let t11204 = F::new(0.36514074074074074075e0) * t11203;
    let t11219 = t241 * t3439;
    let t11243 = F::new(1.0)/pow_3_2::<f64>(t407);
    let t11247 = F::new(28.0) / F::new(27.0) * t11135;
    let t11265 = F::new(1.0) / t410 / t417 / F::new(4.0);
    let t11274 = F::new(1.0) / t3311 / t1097;
    let t11275 = t409 * t11274;
    let t11277 = F::new(1.0) / t3314 / t422;
    (t11204, t11219, t11243, t11247, t11265, t11275, t11277)
}
