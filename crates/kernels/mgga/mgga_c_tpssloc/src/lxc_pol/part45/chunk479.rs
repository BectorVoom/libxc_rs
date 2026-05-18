//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 479/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk479<F: Float>(t300: F, t3407: F, t3369: F, t1143: F, t1166: F, t1156: F, t3375: F, t3377: F, t1164: F, t1147: F, t3395: F, t3400: F) -> (F, F, F, F, F, F) {
    let t3408 = t300 * t3407;
    let t3410 = F::new(0.19751673498613801407e-1) * t300 * t3369;
    let t3411 = t300 * t1143;
    let t3413 = F::new(0.11696447245269292414e1) * t3411 * t1166;
    let t3415 = t3375 * t3377 * t1156;
    let t3417 = F::new(0.11696447245269292414e1) * t1164 * t3415;
    let t3419 = t1147 * t3395 * t1156;
    let t3421 = F::new(0.5848223622634646207e0) * t1164 * t3419;
    let t3422 = t3400 * t3377;
    (t3408, t3410, t3413, t3417, t3421, t3422)
}
