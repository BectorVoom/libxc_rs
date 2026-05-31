//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 594/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk594<F: Float>(t1156: F, t3377: F, t3236: F, t3293: F, t3238: F, t3245: F, t3250: F, t3254: F, t3272: F, t3280: F, t3288: F, t3290: F, t3295: F, t3299: F, t3302: F, t3305: F) -> (F, F) {
    let t3378 = t3377 * t1156;
    let t3383 = F::cast_from(0.40256666666666666667e0_f64) * t3236;
    let t3390 = F::cast_from(0.137975e0_f64) * t3293;
    let t3395 = -F::cast_from(0.1294625e1_f64) * t3272 + F::cast_from(0.258925e1_f64) * t3280 + t3383 - F::cast_from(0.20128333333333333334e0_f64) * t3238 - F::cast_from(0.20128333333333333333e0_f64) * t3245 + F::cast_from(0.60385e0_f64) * t3250 + F::cast_from(0.301925e0_f64) * t3254 + F::cast_from(0.82524375e-1_f64) * t3288 + F::cast_from(0.16504875e0_f64) * t3290 + t3390 - F::cast_from(0.11038e0_f64) * t3295 - F::cast_from(0.27595e-1_f64) * t3299 + F::cast_from(0.16557e0_f64) * t3302 + F::cast_from(0.82785e-1_f64) * t3305;
    (t3378, t3395)
}
