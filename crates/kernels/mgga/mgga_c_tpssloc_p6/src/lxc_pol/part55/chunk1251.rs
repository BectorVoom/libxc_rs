//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1251/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1251<F: Float>(t120210: F, t120254: F, t120292: F, t120328: F, t120548: F, t120582: F, t120613: F, t120652: F, t1390: F, t1983: F, t533: F, t19577: F, t22574: F, t36533: F) -> (F, F) {
    let t120658 = t1983 * t533 * (t120210 + t120254 + t120292 + t120328 + t120548 + t120582 + t120613 + t120652) * t1390;
    let t120663 = F::cast_from(6.0_f64) * t22574 * t36533 * t19577;
    (t120658, t120663)
}
