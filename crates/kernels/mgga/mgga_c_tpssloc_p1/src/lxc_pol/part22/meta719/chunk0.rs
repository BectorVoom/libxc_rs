//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2327/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2327<F: Float>(t59039: F, t16717: F, t58994: F, t59045: F, t59048: F, t39658: F, t46436: F, t46438: F, t67494: F, t67495: F, t67496: F, t67497: F, t67498: F, t67499: F, t67500: F, t67501: F, t67502: F, t67503: F) -> (F, F, F, F, F) {
    let t67504 = F::new(36.0) * t59039;
    let t67506 = F::new(72.0) * t58994 * t16717;
    let t67507 = F::cast_from(0.17544670867903938621e1_f64) * t59045;
    let t67508 = F::cast_from(0.54934341918019635162e-3_f64) * t59048;
    let t67509 = t67494 + t46436 + t46438 + t67495 + t67496 - t39658 + t67497 + t67498 + t67499 + t67500 - t67501 + t67502 + t67503 + t67504 + t67506 - t67507 - t67508;
    (t67504, t67506, t67507, t67508, t67509)
}
