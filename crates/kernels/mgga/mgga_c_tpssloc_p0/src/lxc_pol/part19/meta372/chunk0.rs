//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1380/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1380<F: Float>(t3637: F, t3639: F, t11153: F, t2244: F, t2250: F, t136: F, t3297: F, t11158: F, t9258: F, t3243: F, t1113: F, t11167: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43703 = t3637 * t3637;
    let t43705 = t3639 * t3639;
    let t43706 = F::cast_from(1.0_f64) / t43705;
    let t43711 = t11153 * t2244 * t2250;
    let t43713 = t136 * t3297 * t43711;
    let t43715 = t11158 * t9258;
    let t43717 = t136 * t3297 * t43715;
    let t43719 = t3243 * t2250;
    let t43721 = t136 * t1113 * t43719;
    let t43723 = t11167 * t9258;
    (t43703, t43706, t43711, t43713, t43715, t43717, t43719, t43721, t43723)
}
