//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 845/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk845<F: Float>(t3477: F, t974: F, t1174: F, t3430: F, t3433: F, t3436: F, t3443: F, t3447: F, t3452: F, t3457: F, t3461: F, t3472: F) -> (F, F) {
    let t3478 = t974 * t3477;
    let t3481 = -t3430 - F::cast_from(0.18518518518518518518e-3_f64) * t3433 - F::cast_from(0.55555555555555555554e-3_f64) * t3436 + F::cast_from(0.37037037037037037036e-3_f64) * t1174 * t3443 + F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t3452 - F::cast_from(0.55555555555555555554e-3_f64) * t1174 * t3457 - F::cast_from(0.27777777777777777777e-3_f64) * t1174 * t3461 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t3472 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t3478;
    (t3478, t3481)
}
