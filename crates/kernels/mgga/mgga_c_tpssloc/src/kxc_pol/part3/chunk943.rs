//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 943/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk943<F: Float>(t1128: F, t3324: F, t1124: F, t3356: F, t3355: F, t432: F, t427: F, t1094: F, t3263: F, t3395: F, t3403: F, t11135: F) -> (F, F, F, F, F, F) {
    let t11410 = t3324 * t1128;
    let t11415 = t1124 * t3356;
    let t11419 = F::new(1.0) / t3355 / t432;
    let t11420 = t427 * t11419;
    let t11424 = t1094 * t3263;
    let t11433 = t3395 * t3403;
    let t11444 = F::cast_from(0.53272592592592592592e-1_f64) * t11135;
    (t11410, t11415, t11420, t11424, t11433, t11444)
}
