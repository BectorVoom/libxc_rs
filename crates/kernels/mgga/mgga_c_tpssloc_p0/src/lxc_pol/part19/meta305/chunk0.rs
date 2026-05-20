//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1092/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1092<F: Float>(t39033: F, t587: F, t598: F, t14: F, t2230: F, t594: F, t9223: F, t22811: F, t19: F, t604: F, t9226: F, t2233: F, t2239: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39034 = F::new(1638.0) * t39033;
    let t39035 = t587 * t598;
    let t39036 = F::new(0.74688e4) * t39035;
    let t39037 = t14 * t2230;
    let t39038 = F::new(0.175056e5) * t39037;
    let t39039 = t594 * t9223;
    let t39040 = F::new(0.1822464e5) * t39039;
    let t39041 = F::new(1.0) / t22811;
    let t39043 = F::new(0.683424e4) * t19 * t39041;
    let t39046 = t9226 * t604;
    let t39049 = t2233 * t2239;
    (t39034, t39035, t39036, t39037, t39038, t39040, t39043, t39046, t39049)
}
