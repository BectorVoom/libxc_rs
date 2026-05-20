//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1695/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1695<F: Float>(t22674: F, t6907: F, t6897: F, t131: F, t557: F, t209: F, t1878: F, t212: F, t225: F) -> (F, F, F, F, F, F) {
    let t22675 = t22674 * t6907;
    let t22676 = t6897 * t22675;
    let t22683 = t557 * t131;
    let t22684 = t22683 * t209;
    let t22685 = t1878 * t22684;
    let t22690 = t212 * t225;
    (t22675, t22676, t22683, t22684, t22685, t22690)
}
