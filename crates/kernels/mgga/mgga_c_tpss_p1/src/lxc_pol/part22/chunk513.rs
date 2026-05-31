//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 513/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk513<F: Float>(t114: F, t2100: F, t630: F, t2069: F, t2070: F, t2075: F, t69: F) -> (F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t2101 = t630 * t2100;
    let t2105 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t2069 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2070 + t69 * t2075 / F::cast_from(4.0_f64) - t69 * t2101 / F::cast_from(8.0_f64));
    (t2101, t2105)
}
