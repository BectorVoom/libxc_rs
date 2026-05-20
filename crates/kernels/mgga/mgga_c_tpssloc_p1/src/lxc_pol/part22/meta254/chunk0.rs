//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1374/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1374<F: Float>(t11135: F, t10292: F, t281: F, t415: F, t1114: F, t2403: F, t241: F, t3439: F, t407: F, t410: F, t417: F, t1097: F, t3311: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11195 = F::cast_from(0.93011851851851851854e0_f64) * t11135;
    let t11203 = t281 * t10292 * t415;
    let t11204 = F::cast_from(0.36514074074074074075e0_f64) * t11203;
    let t11211 = t2403 * t1114;
    let t11219 = t241 * t3439;
    let t11243 = F::new(1.0)/pow_3_2::<F>(t407);
    let t11247 = F::new(28.0) / F::new(27.0) * t11135;
    let t11265 = F::new(1.0) / t410 / t417 / F::new(4.0);
    let t11274 = F::new(1.0) / t3311 / t1097;
    (t11195, t11203, t11204, t11211, t11219, t11243, t11247, t11265, t11274)
}
