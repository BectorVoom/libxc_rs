//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 674/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk674<F: Float>(t1166: F, t3411: F, t1156: F, t3375: F, t3377: F, t1164: F, t1147: F, t3395: F, t3400: F, t3403: F, t457: F, t697: F) -> (F, F, F, F, F, F, F, F) {
    let t3413 = F::cast_from(0.11696447245269292414e1_f64) * t3411 * t1166;
    let t3415 = t3375 * t3377 * t1156;
    let t3417 = F::cast_from(0.11696447245269292414e1_f64) * t1164 * t3415;
    let t3419 = t1147 * t3395 * t1156;
    let t3421 = F::cast_from(0.5848223622634646207e0_f64) * t1164 * t3419;
    let t3422 = t3400 * t3377;
    let t3423 = t3422 * t3403;
    let t3425 = F::cast_from(0.17315859105681463759e2_f64) * t1164 * t3423;
    let t3426 = t697 * t457;
    (t3413, t3415, t3417, t3419, t3421, t3423, t3425, t3426)
}
