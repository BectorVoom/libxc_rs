//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 555/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk555<F: Float>(t1156: F, t3377: F, t3236: F, t3293: F, t3238: F, t3245: F, t3250: F, t3254: F, t3272: F, t3280: F, t3288: F, t3290: F, t3295: F, t3299: F, t3302: F, t3305: F) -> (F, F) {
    let t3378 = t3377 * t1156;
    let t3383 = F::cast_from(0.40256666666666666667e0_f64) * t3236;
    let t3390 = F::new(0.137975e0) * t3293;
    let t3395 = -F::new(0.1294625e1) * t3272 + F::new(0.258925e1) * t3280 + t3383 - F::cast_from(0.20128333333333333334e0_f64) * t3238 - F::cast_from(0.20128333333333333333e0_f64) * t3245 + F::new(0.60385e0) * t3250 + F::new(0.301925e0) * t3254 + F::new(0.82524375e-1) * t3288 + F::new(0.16504875e0) * t3290 + t3390 - F::new(0.11038e0) * t3295 - F::new(0.27595e-1) * t3299 + F::new(0.16557e0) * t3302 + F::new(0.82785e-1) * t3305;
    (t3378, t3395)
}
