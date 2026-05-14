//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1154/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1154<F: Float>(t82069: F, t225: F, t23228: F, t6563: F, t81597: F, t1882: F, t81686: F, t9537: F, t1883: F, t82045: F, t10109: F, t1914: F, t40772: F, t3034: F, t336: F, t221: F, t697: F) -> (F, F, F, F, F, F, F, F, F) {
    let t82070 = 0.98696044010893586188e-1 * t82069;
    let t82074 = t23228 * t225;
    let t82122 = t81597 * t6563;
    let t82123 = 0.16220877603642232915e0 * t82122;
    let t82153 = t81686 * t9537 * t1882;
    let t82154 = 0.13707783890401886971e-2 * t82153;
    let t82218 = t82045 * t1883;
    let t82219 = 0.27720185200590482541e0 * t82218;
    let t82252 = t225 * t10109;
    let t82312 = t1914 * t40772;
    let t82510 = 1.0 / t3034 / t336;
    let t82631 = t221 * t697;
    (t82070, t82074, t82123, t82154, t82219, t82252, t82312, t82510, t82631)
}
