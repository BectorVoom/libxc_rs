//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 867/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk867<F: Float>(t3469: F, t457: F, t460: F, t974: F, t1184: F, t1174: F, t3430: F, t3433: F, t3436: F, t3443: F, t3447: F, t3452: F, t3457: F, t3461: F) -> (F, F, F, F, F, F) {
    let t3470 = t457 * t3469;
    let t3471 = t3470 * t460;
    let t3472 = t974 * t3471;
    let t3475 = t1184 * t1184;
    let t3477 = t457 * t3475 * t460;
    let t3478 = t974 * t3477;
    let t3481 = -t3430 - F::cast_from(0.18518518518518518518e-3_f64) * t3433 - F::cast_from(0.55555555555555555554e-3_f64) * t3436 + F::cast_from(0.37037037037037037036e-3_f64) * t1174 * t3443 + F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t3452 - F::cast_from(0.55555555555555555554e-3_f64) * t1174 * t3457 - F::cast_from(0.27777777777777777777e-3_f64) * t1174 * t3461 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t3472 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t3478;
    (t3471, t3472, t3475, t3477, t3478, t3481)
}
