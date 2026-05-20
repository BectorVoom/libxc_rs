//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2754/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2754<F: Float>(t40738: F, t40745: F, t46283: F, t46285: F, t13133: F, t4202: F, t5597: F, t9912: F, t40754: F, t40761: F, t46291: F, t40741: F, t40743: F, t40748: F, t40760: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t58025 = F::cast_from(0.43374325201206959367e-1_f64) * t40738;
    let t58026 = F::cast_from(0.10843581300301739842e-1_f64) * t40745;
    let t58027 = F::new(48.0) * t46283;
    let t58028 = F::new(8.0) * t46285;
    let t58030 = F::new(16.0) * t13133 * t4202;
    let t58032 = F::new(4.0) * t9912 * t5597;
    let t58033 = F::cast_from(0.20779030926817756511e3_f64) * t40754;
    let t58034 = F::cast_from(0.20508037716432813316e4_f64) * t40761;
    let t58035 = F::new(48.0) * t46291;
    let t58036 = -t58025 - t40741 - t40743 + t58026 + t58027 + t58028 + t40748 + t58030 + t58032 + t58033 + t40760 - t58034 + t58035;
    (t58025, t58026, t58027, t58028, t58030, t58032, t58033, t58034, t58035, t58036)
}
