//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 383/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk383<F: Float>(t2663: F, t756: F, t68: F, t845: F, t20: F, t61: F, t241: F, t244: F, t248: F, t238: F, t835: F, t841: F) -> (F, F, F, F, F, F, F) {
    let t2665 = F::new(0.24415263074675393405e-3) * t756 * t2663;
    let t2671 = t68 * t845;
    let t2690 = F::new(1.0) / t61 / t20;
    let t2691 = t2690 * t241;
    let t2693 = t2691 * t244 * t248;
    let t2695 = F::new(119.0) / F::new(13824.0) * t238 * t2693;
    let t2696 = t841 * t835;
    (t2665, t2671, t2690, t2691, t2693, t2695, t2696)
}
