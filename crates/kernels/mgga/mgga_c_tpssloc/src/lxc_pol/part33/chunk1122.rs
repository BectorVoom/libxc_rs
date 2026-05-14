//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1122/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1122<F: Float>(t82153: F, t1883: F, t82045: F, t10109: F, t225: F, t1914: F, t40772: F, t1054: F, t2775: F, t10213: F, t344: F, t381: F, t2770: F, t10189: F, t1926: F, t221: F) -> (F, F, F, F, F, F, F, F, F) {
    let t82154 = 0.13707783890401886971e-2 * t82153;
    let t82218 = t82045 * t1883;
    let t82219 = 0.27720185200590482541e0 * t82218;
    let t82252 = t225 * t10109;
    let t82312 = t1914 * t40772;
    let t82342 = t1054 * t2775;
    let t82390 = t10213 * t344;
    let t82391 = t82390 * t381;
    let t82411 = t1054 * t2770;
    let t82431 = t1926 * t221 * t10189;
    (t82154, t82219, t82252, t82312, t82342, t82390, t82391, t82411, t82431)
}
