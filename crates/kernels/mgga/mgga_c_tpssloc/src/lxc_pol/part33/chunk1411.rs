//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1411/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1411<F: Float>(t105208: F, t106892: F, t107493: F, t107543: F, t1873: F, t20347: F, t3941: F, t1458: F, t28017: F, t5493: F, t7467: F, t75784: F) -> (F, F, F, F, F) {
    let t107545 = t105208 + t106892 + t107493 + t107543;
    let t107552 = F::new(27.0) * t3941 * t1873 * t20347;
    let t107555 = F::new(81.0) * t3941 * t28017 * t1458;
    let t107558 = F::new(81.0) * t3941 * t7467 * t5493;
    let t107566 = F::new(0.135e2) * t75784 * t1873;
    (t107545, t107552, t107555, t107558, t107566)
}
