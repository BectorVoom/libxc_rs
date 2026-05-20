//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1533/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1533<F: Float>(t761: F, t9713: F, t172: F, t2448: F, t763: F, t177: F, t2508: F) -> (F, F, F, F) {
    let t9715 = F::cast_from(0.5848223622634646207e0_f64) * t761 * t9713;
    let t9716 = t2448 * t172;
    let t9717 = t9716 * t763;
    let t9720 = F::new(1.0) / t2508 / t177;
    (t9715, t9716, t9717, t9720)
}
