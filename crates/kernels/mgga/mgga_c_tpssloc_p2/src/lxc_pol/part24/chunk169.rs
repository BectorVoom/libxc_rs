//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 169/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk169<F: Float>(t466: F, t491: F, t477: F, t68: F, t470: F, t254: F, t193: F, t336: F, t425: F, t453: F, t455: F, t265: F) -> (F, F, F, F, F, F, F) {
    let t492 = t466 * t491;
    let t493 = t68 * t477;
    let t494 = t493 * t491;
    let t496 = t470 * t494 + F::cast_from(1.0_f64);
    let t497 = F::cast_from(1.0_f64) / t496;
    let t498 = t254 * t497;
    let t500 = t492 * t498 + F::cast_from(1.0_f64);
    let t501 = F::ln(t500);
    let t504 = t193 * t336 * t501 - t425 + t453 + t455;
    let t505 = t265 < t504;
    (t492, t493, t494, t496, t498, t500, t504)
}
