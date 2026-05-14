//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 477/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk477<F: Float>(t118: F, t776: F, t794: F, t2576: F, t210: F, t214: F, t2553: F, t59: F, t835: F, t154: F, t116: F, t206: F, t212: F, t2562: F, t2564: F, t2569: F, t2571: F, t2573: F, t787: F) -> (F, F, F, F, F, F, F) {
    let t2578 = t118 * t794 * t776;
    let t2579 = t2576 * t2578;
    let t2582 = t210 * t214 * t2553;
    let t2585 = t59 * t835;
    let t2586 = t2585 * t154;
    let t2587 = t206 * t116;
    let t2588 = t2587 * t212;
    let t2590 = 0.83333333333333333332e-3 * t2586 * t2588;
    let t2591 = t2562 + 0.77777777777777777775e-2 * t2564 + t2569 + 0.49999999999999999998e-2 * t2571 * t2573 + 0.16666666666666666666e-2 * t2579 - 0.16666666666666666666e-2 * t787 * t2582 - t2590;
    (t2578, t2582, t2585, t2586, t2587, t2588, t2591)
}
