//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1197/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1197<F: Float>(t114240: F, t114242: F, t114172: F, t6897: F, t7700: F, t22674: F, t32697: F, t114253: F, t114255: F, t2007: F, t254: F, t114278: F, t32694: F, t6914: F, t114291: F, t32735: F, t6883: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t120561 = 0.16449340668482264365e-1 * t114240;
    let t120566 = 0.38381794893125283518e-1 * t114242;
    let t120568 = t6897 * t114172 * t7700;
    let t120569 = 0.82246703342411321825e-2 * t120568;
    let t120576 = t6897 * t22674 * t32697;
    let t120577 = 0.82246703342411321825e-2 * t120576;
    let t120579 = 0.38381794893125283518e-1 * t114253;
    let t120590 = 0.76763589786250567036e-1 * t114255;
    let t120591 = t2007 * t254;
    let t120594 = 0.16449340668482264365e-1 * t114278;
    let t120605 = t6914 * t32694;
    let t120606 = 0.76763589786250567037e-1 * t120605;
    let t120607 = 0.38381794893125283518e-1 * t114291;
    let t120610 = t6883 * t32735;
    (t120561, t120566, t120569, t120577, t120579, t120590, t120591, t120594, t120606, t120607, t120610)
}
