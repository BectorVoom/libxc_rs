//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 848/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk848<F: Float>(t422: F, t3265: F, t3313: F, t3236: F, t3238: F, t3245: F, t3250: F, t3254: F, t1124: F, t1128: F, t1127: F, t432: F) -> (F, F, F, F, F, F, F, F) {
    let t3314 = t422 * t422;
    let t3315 = F::new(1.0) / t3314;
    let t3316 = t3265 * t3315;
    let t3318 = F::cast_from(0.16081979498692535067e2_f64) * t3313 * t3316;
    let t3319 = F::cast_from(0.22831111111111111111e-1_f64) * t3236;
    let t3324 = t3319 - F::cast_from(0.11415555555555555555e-1_f64) * t3238 - F::cast_from(0.11415555555555555555e-1_f64) * t3245 + F::cast_from(0.34246666666666666666e-1_f64) * t3250 + F::cast_from(0.17123333333333333333e-1_f64) * t3254;
    let t3327 = t1124 * t1128;
    let t3330 = t1127 * t432;
    (t3314, t3315, t3316, t3318, t3319, t3324, t3327, t3330)
}
