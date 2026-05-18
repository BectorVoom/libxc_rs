//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1208/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1208<F: Float>(t19164: F, t19207: F, t1241: F, t1235: F, t6150: F, t1760: F, t5088: F, t3598: F, t1251: F, t6267: F, t6243: F, t11606: F) -> (F, F, F, F, F) {
    let t19208 = t19164 + t19207;
    let t19209 = t1241 * t19208;
    let t19211 = t6150 * t1235;
    let t19213 = t1760 * t5088;
    let t19214 = t3598 * t19213;
    let t19219 = t6267 * t1251;
    let t19220 = t3598 * t19219;
    let t19225 = t6243 * t1251;
    let t19226 = t11606 * t19225;
    (t19209, t19211, t19214, t19220, t19226)
}
