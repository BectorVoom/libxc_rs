//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 507/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk507<F: Float>(t221: F, t3427: F, t456: F, t1176: F, t135: F, t1179: F, t1174: F, t1186: F, t1089: F, t405: F, t974: F, t337: F, t51: F) -> (F, F, F, F, F, F, F) {
    let t3428 = t221 * t3427;
    let t3430 = F::cast_from(0.18518518518518518518e-3_f64) * t456 * t3428;
    let t3431 = t135 * t1176;
    let t3432 = t3431 * t1179;
    let t3433 = t1174 * t3432;
    let t3435 = t135 * t1186;
    let t3436 = t1174 * t3435;
    let t3439 = F::cast_from(1.0_f64) / t405 / t1089;
    let t3440 = t974 * t3439;
    let t3446 = t51 * t337;
    (t3430, t3431, t3433, t3436, t3439, t3440, t3446)
}
