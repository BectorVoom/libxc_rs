//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1203/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1203<F: Float>(t40808: F, t2655: F, t9912: F, t2745: F, t2528: F, t9716: F, t193: F, t202: F, t2752: F, t39549: F, t39563: F, t40793: F, t40795: F, t40797: F, t40799: F, t40801: F, t40803: F, t40805: F, t40807: F) -> (F, F, F, F) {
    let t40809 = F::new(48.0) * t40808;
    let t40811 = F::new(24.0) * t9912 * t2655;
    let t40812 = t2745 * t2745;
    let t40817 = t9716 * t2528;
    let t40818 = F::cast_from(0.10389515463408878255e3_f64) * t40817;
    let t40819 = -F::new(3.0) * t193 * t202 * t2752 * t40812 + t39549 + t39563 + t40793 + t40795 + t40797 + t40799 + t40801 - t40803 - t40805 + t40807 + t40809 + t40811 - t40818;
    (t40809, t40811, t40818, t40819)
}
