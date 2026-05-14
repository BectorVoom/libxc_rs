//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 458/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk458<F: Float>(t152: F, t32: F, t181: F, t204: F, t686: F, t756: F, t20: F, t61: F, t241: F, t244: F, t248: F, t238: F, t835: F, t841: F, t812: F, t849: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2658 = t32 * t152;
    let t2663 = t686 * t204 * t181;
    let t2665 = 0.24415263074675393405e-3 * t756 * t2663;
    let t2690 = 1.0 / t61 / t20;
    let t2691 = t2690 * t241;
    let t2693 = t2691 * t244 * t248;
    let t2695 = 119.0 / 13824.0 * t238 * t2693;
    let t2696 = t841 * t835;
    let t2697 = t812 * t2696;
    let t2698 = t2697 * t849;
    (t2658, t2663, t2665, t2690, t2691, t2693, t2695, t2697, t2698)
}
