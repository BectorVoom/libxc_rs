//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1131/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1131<F: Float>(t39634: F, t1287: F, t9218: F, t118: F, t142: F, t39283: F) -> (F, F, F) {
    let t39635 = F::new(96.0) * t39634;
    let t39655 = F::new(480.0) * t9218 * t1287;
    let t39658 = F::cast_from(0.11483599538271604938e-1_f64) * t118 * t39283 * t142;
    (t39635, t39655, t39658)
}
