//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1395/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1395<F: Float>(t13563: F, t13566: F, t13602: F, t2932: F, t4471: F, t300: F, t4446: F, t3053: F, t4644: F, t10422: F, t4578: F, t3070: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14352 = F::cast_from(0.41203703703703703704e-2_f64) * t13563;
    let t14353 = F::cast_from(0.12361111111111111111e-1_f64) * t13566;
    let t14354 = F::cast_from(0.61805555555555555556e-2_f64) * t13602;
    let t14409 = F::cast_from(0.2283111111111111111e-1_f64) * t13566;
    let t14410 = F::cast_from(0.11415555555555555555e-1_f64) * t13602;
    let t14459 = t4471 * t2932;
    let t14473 = t300 * t4446;
    let t14495 = t4644 * t3053 / F::new(3456.0);
    let t14501 = t10422 * t4578;
    let t14503 = t3070 * t14501 / F::new(3456.0);
    (t14352, t14353, t14354, t14409, t14410, t14459, t14473, t14495, t14501, t14503)
}
