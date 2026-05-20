//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1392/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1392<F: Float>(t11349: F, t427: F, t3358: F, t435: F, t1147: F, t3368: F, t1143: F, t3400: F, t11292: F, t440: F, t11135: F, t11203: F) -> (F, F, F, F, F, F, F) {
    let t11350 = t427 * t11349;
    let t11352 = F::new(1.0) / t3358 / t435;
    let t11356 = t3368 * t1147;
    let t11361 = t1143 * t3400;
    let t11365 = t440 * t11292;
    let t11369 = F::cast_from(0.93932222222222222223e0_f64) * t11135;
    let t11372 = F::cast_from(0.36793333333333333333e0_f64) * t11203;
    (t11350, t11352, t11356, t11361, t11365, t11369, t11372)
}
