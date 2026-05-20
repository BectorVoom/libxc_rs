//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2069/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2069<F: Float>(t7607: F, t82573: F, t1920: F, t25766: F, t968: F, t23384: F, t25739: F, t25751: F, t82431: F, t4657: F, t6703: F, t7554: F) -> (F, F, F, F, F, F) {
    let t89546 = F::cast_from(0.14621636149762012769e-1_f64) * t82573 * t7607;
    let t89561 = F::cast_from(0.54831135561607547884e-2_f64) * t1920 * t968 * t25766;
    let t89583 = F::cast_from(0.10966227112321509577e-1_f64) * t23384 * t25739;
    let t89597 = F::cast_from(0.18277045187202515961e-2_f64) * t82431 * t25751;
    let t89598 = t6703 * t4657;
    let t89609 = t82573 * t7554;
    (t89546, t89561, t89583, t89597, t89598, t89609)
}
