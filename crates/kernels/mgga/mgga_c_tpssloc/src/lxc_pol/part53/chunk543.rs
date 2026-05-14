//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 543/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk543<F: Float>(t1894: F, t236: F, t776: F, t6591: F, t2229: F, t61: F, t1891: F, t133: F, t119: F, t212: F, t1895: F, t213: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6593 = t1894 * t236 * t776;
    let t6594 = t6591 * t6593;
    let t6597 = 1.0 / t61 / t2229;
    let t6598 = t6597 * t1891;
    let t6599 = t6598 * t133;
    let t6600 = t119 * t212;
    let t6601 = t6600 * t1895;
    let t6602 = t6599 * t6601;
    let t6604 = t213 * t225;
    (t6593, t6594, t6597, t6598, t6599, t6600, t6601, t6602, t6604)
}
