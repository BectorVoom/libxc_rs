//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 48/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk48<F: Float>(t118: F, t121: F, t124: F) -> (F, F) {
    let t127 = F::cast_from(1.0_f64) + F::cast_from(0.66523565010354492023e-2_f64) * t118 + F::cast_from(0.44253847016868604463e-4_f64) * t121 * t124;
    let t128 = F::cast_from(1.0_f64) / t127;
    (t127, t128)
}
