//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 507/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk507<F: Float>(t236: F, t2627: F, t232: F, t815: F, t835: F, t812: F, t831: F, t242: F, t67: F, t845: F, t246: F, t120: F, t828: F, t776: F, t753: F, t758: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2628 = t2627 * t236;
    let t2632 = t232 * t232;
    let t2638 = t815 * t835;
    let t2639 = t812 * t2638;
    let t2640 = t2639 * t831;
    let t2642 = t815 * t242;
    let t2643 = t812 * t2642;
    let t2644 = t845 * t67;
    let t2645 = t2644 * t246;
    let t2646 = t120 * t828;
    let t2647 = t232 * t776;
    let t2652 = t753 * t67;
    let t2653 = t2652 * t758;
    (t2628, t2632, t2639, t2640, t2643, t2645, t2646, t2647, t2653)
}
