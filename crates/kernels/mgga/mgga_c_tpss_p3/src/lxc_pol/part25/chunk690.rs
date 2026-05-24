//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 690/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk690<F: Float>(t4416: F, t4417: F, t4415: F, t1642: F, t3267: F, t3275: F, t3273: F, t3179: F, t3191: F, t189: F, t4377: F, t489: F) -> (F, F, F, F, F, F, F) {
    let t4418 = t4416 * t4417;
    let t4419 = t4415 * t4418;
    let t4422 = t3267 * t1642;
    let t4424 = t4416 * t3275;
    let t4425 = t3273 * t4424;
    let t4428 = F::cast_from(0.5848223622634646207e0_f64) * t3179;
    let t4429 = F::cast_from(0.18311447306006545054e-3_f64) * t3191;
    let t4430 = t4377 * t189;
    let t4431 = t489 * t4430;
    (t4419, t4422, t4425, t4428, t4429, t4430, t4431)
}
