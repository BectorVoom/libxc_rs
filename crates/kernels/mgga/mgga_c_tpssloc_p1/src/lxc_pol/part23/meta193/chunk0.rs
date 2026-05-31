//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 829/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk829<F: Float>(t3311: F, t419: F, t409: F, t11135: F, t10292: F, t281: F, t415: F, t241: F, t3439: F, t407: F, t410: F, t417: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11189 = F::cast_from(1.0_f64) / t3311 / t419;
    let t11190 = t409 * t11189;
    let t11195 = F::cast_from(0.93011851851851851854e0_f64) * t11135;
    let t11203 = t281 * t10292 * t415;
    let t11204 = F::cast_from(0.36514074074074074075e0_f64) * t11203;
    let t11219 = t241 * t3439;
    let t11243 = F::cast_from(1.0_f64)/pow_3_2::<F>(t407);
    let t11247 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t11135;
    let t11265 = F::cast_from(1.0_f64) / t410 / t417 / F::cast_from(4.0_f64);
    (t11189, t11190, t11195, t11203, t11204, t11219, t11243, t11247, t11265)
}
