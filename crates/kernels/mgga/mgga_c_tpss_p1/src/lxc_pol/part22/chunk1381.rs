//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1381/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1381<F: Float>(t114: F, t61871: F, t61874: F, t61876: F, t63006: F, t65447: F, t65450: F, t65453: F, t65455: F, t67531: F, t67532: F, t67533: F, t116: F, t20287: F) -> (F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t67537 = -t63006 - F::cast_from(44.0_f64) / F::cast_from(9.0_f64) * t61871 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t61874 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t61876 - t67531 - t67532 + t67533 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t65447 + t65450 + t65453 / F::cast_from(2.0_f64) - t65455 / F::cast_from(4.0_f64);
    let t67538 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t67537);
    let t67541 = t20287 * t116;
    (t67538, t67541)
}
