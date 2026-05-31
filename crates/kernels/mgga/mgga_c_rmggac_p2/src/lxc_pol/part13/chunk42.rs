//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 42/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk42<F: Float>(t107: F, t27: F, t29: F, t38: F) -> (F, F, F, F) {
    let t114 = F::cast_from(0.8e-1_f64) + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t107 * t29 * t27 + F::cast_from(0.125e-1_f64) * t38;
    let t115 = t114 * t114;
    let t116 = t115 * t114;
    let t117 = F::cast_from(1.0_f64) / t116;
    (t114, t115, t116, t117)
}
