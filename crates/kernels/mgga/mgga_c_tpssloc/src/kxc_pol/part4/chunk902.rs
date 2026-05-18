//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 902/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk902<F: Float>(t11135: F, t11203: F, t1127: F, t3355: F, t427: F, t3358: F, t435: F, t1143: F, t3400: F, t11292: F, t440: F, t1124: F, t3356: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11314 = F::new(0.16068111111111111111e1) * t11135;
    let t11317 = F::new(0.46308888888888888888e0) * t11203;
    let t11349 = F::new(1.0) / t3355 / t1127;
    let t11350 = t427 * t11349;
    let t11352 = F::new(1.0) / t3358 / t435;
    let t11361 = t1143 * t3400;
    let t11365 = t440 * t11292;
    let t11369 = F::new(0.93932222222222222223e0) * t11135;
    let t11372 = F::new(0.36793333333333333333e0) * t11203;
    let t11415 = t1124 * t3356;
    (t11314, t11317, t11350, t11352, t11361, t11365, t11369, t11372, t11415)
}
