//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 426/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk426<F: Float>(t207: F, t215: F, t2559: F, t782: F, t786: F, t59: F, t591: F, t795: F, t154: F, t244: F, t205: F, t792: F, t835: F, t116: F, t206: F, t212: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2562 = 0.64814814814814814813e-2 * t2559 * t207 * t215;
    let t2563 = t782 * t786;
    let t2566 = t59 * t591;
    let t2569 = 0.26388888888888888888e-2 * t2566 * t207 * t795;
    let t2570 = t154 * t244;
    let t2571 = t205 * t2570;
    let t2576 = t792 * t786;
    let t2585 = t59 * t835;
    let t2586 = t2585 * t154;
    let t2587 = t206 * t116;
    let t2588 = t2587 * t212;
    (t2562, t2563, t2566, t2569, t2570, t2571, t2576, t2585, t2586, t2587, t2588)
}
