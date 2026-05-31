//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1105/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1105<F: Float>(t17635: F, t4583: F, t4582: F, t1041: F, t13948: F, t13952: F, t13959: F, t13963: F, t13966: F, t13972: F, t17616: F, t17621: F, t17625: F, t17632: F, t2960: F, t3039: F, t5885: F, t5890: F, t5894: F) -> F {
    let t17636 = t4583 * t17635;
    let t17637 = t4582 * t17636;
    let t17640 = t17616 / F::cast_from(864.0_f64) - t2960 * t5894 / F::cast_from(81.0_f64) + t17621 / F::cast_from(648.0_f64) + t13948 + t13952 + t13959 + t13963 - t13966 / F::cast_from(6912.0_f64) - t17625 / F::cast_from(432.0_f64) - t2960 * t5890 / F::cast_from(108.0_f64) + t2960 * t5885 / F::cast_from(54.0_f64) - t3039 * t17632 / F::cast_from(1536.0_f64) - t1041 * t17637 / F::cast_from(2304.0_f64) - t13972;
    t17640
}
