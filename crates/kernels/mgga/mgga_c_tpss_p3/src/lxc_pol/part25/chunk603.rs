//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 603/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk603<F: Float>(t3518: F, t555: F, t1329: F, t2091: F, t636: F, t108: F, t2: F, t105: F, t1325: F, t1327: F, t3515: F, t631: F, t637: F, t97: F) -> (F, F, F) {
    let t3519 = t3518 * t555;
    let t3524 = t2091 * t1329;
    let t3525 = t3524 * t636;
    let t3528 = t108 * t2;
    let t3529 = t3528 * t555;
    let t3532 = -F::cast_from(25.0_f64) / F::cast_from(9.0_f64) * t631 * t1325 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t97 * t3515 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t97 * t3519 - F::cast_from(25.0_f64) / F::cast_from(9.0_f64) * t1327 * t637 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t105 * t3525 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t105 * t3529;
    (t3525, t3529, t3532)
}
