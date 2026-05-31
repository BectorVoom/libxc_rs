//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 973/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk973<F: Float>(t13406: F, t38: F, t4573: F, t7771: F, t2033: F, t4579: F, t7780: F, t2040: F, t13335: F, t3431: F, t3472: F, t3477: F, t581: F, t608: F, t612: F) -> (F, F) {
    let t13407 = t38 * t13406;
    let t13422 = t7771 * t4573;
    let t13427 = t2033 * t4579;
    let t13432 = t7780 * t4573;
    let t13437 = t2040 * t4579;
    let t13442 = -F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t13422 * t581 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t3472 * t3431 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t13427 * t581 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t608 * t13335 + F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t13432 * t581 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t3477 * t3431 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t13437 * t581 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t612 * t13335;
    (t13407, t13442)
}
