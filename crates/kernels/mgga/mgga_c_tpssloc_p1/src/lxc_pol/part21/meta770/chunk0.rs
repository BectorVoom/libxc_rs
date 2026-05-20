//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2670/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2670<F: Float>(t53796: F, t53798: F, t39305: F, t1799: F, t3914: F, t12477: F, t20067: F, t3734: F, t3918: F, t39261: F, t39266: F, t39304: F, t39309: F, t39312: F, t39316: F, t5126: F, t5161: F, t6330: F) -> (F, F, F, F) {
    let t56114 = F::cast_from(0.46785788981077169656e1_f64) * t53796;
    let t56115 = F::cast_from(0.70178683471615754484e1_f64) * t53798;
    let t56119 = F::cast_from(0.20779030926817756511e3_f64) * t39305;
    let t56120 = t1799 * t3914;
    let t56124 = -F::new(6.0) * t12477 * t5126 * t6330 + F::new(6.0) * t20067 * t3734 * t5126 - F::new(6.0) * t3918 * t5161 * t56120 - t39261 - t39266 - t39304 - t39309 + t39312 + t39316 + t56114 - t56115 + t56119;
    (t56114, t56115, t56119, t56124)
}
