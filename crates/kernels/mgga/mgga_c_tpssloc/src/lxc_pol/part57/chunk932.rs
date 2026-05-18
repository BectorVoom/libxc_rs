//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 932/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk932<F: Float>(t1799: F, t22690: F, t22792: F, t6950: F, t22779: F, t32714: F, t1814: F, t31175: F, t8467: F, t2006: F, t32745: F, t6914: F) -> (F, F, F, F, F) {
    let t120393 = t22792 * t22690 * t6950 * t1799;
    let t120410 = t22779 * t32714;
    let t120416 = t1814 * t31175 * t8467;
    let t120437 = t2006 * t1799;
    let t120446 = t6914 * t32745;
    (t120393, t120410, t120416, t120437, t120446)
}
