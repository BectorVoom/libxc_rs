//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 792/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk792<F: Float>(t74283: F, t1971: F, t2144: F, t3351: F, t41006: F, t68422: F, t68440: F, t9122: F, t2367: F, t352: F, t875: F, t14025: F, t21713: F, t40167: F, t9212: F) -> (F, F, F, F, F, F) {
    let t74284 = F::cast_from(0.24829349937757072983e-4_f64) * t74283;
    let t74287 = t3351 * t1971 * t2144 * t41006;
    let t74290 = t68440 * t68422 * t9122;
    let t74292 = t2367 * t352;
    let t74295 = t3351 * t1971 * t875 * t74292;
    let t74299 = t21713 * t14025 * t40167 * t9212;
    (t74284, t74287, t74290, t74292, t74295, t74299)
}
