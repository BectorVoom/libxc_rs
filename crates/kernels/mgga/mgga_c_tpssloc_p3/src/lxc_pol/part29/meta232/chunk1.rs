//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1089/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1089<F: Float>(t210: F, t214: F, t5187: F, t1315: F, t3725: F, t3727: F, t3731: F, t3742: F, t3751: F, t5192: F, t5195: F, t5198: F, t5203: F) -> (F, F) {
    let t5206 = t210 * t214 * t5187;
    let t5210 = t3725 + F::cast_from(0.38888888888888888888e-2_f64) * t3727 + t3731 + F::cast_from(0.38888888888888888887e-2_f64) * t5192 + F::cast_from(0.49999999999999999998e-2_f64) * t5195 * t5198 + F::cast_from(0.8333333333333333333e-3_f64) * t5203 - F::cast_from(0.16666666666666666666e-2_f64) * t1315 * t5206 + F::cast_from(0.83333333333333333332e-3_f64) * t3742 - t3751;
    (t5206, t5210)
}
