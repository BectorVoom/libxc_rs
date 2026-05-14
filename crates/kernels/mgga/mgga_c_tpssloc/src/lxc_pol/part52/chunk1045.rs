//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1045/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1045<F: Float>(t30633: F, t865: F, t23270: F, t1888: F, t794: F, t8331: F, t6562: F, t225: F, t258: F, t6624: F, t214: F, t1880: F, t8362: F, t2718: F, t8352: F, t10110: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30634 = t30633 * t865;
    let t30635 = t23270 * t30634;
    let t30637 = 0.3289868133696452873e-1 * t1888 * t30635;
    let t30638 = t794 * t8331;
    let t30640 = 0.82246703342411321825e-2 * t6562 * t30638;
    let t30642 = t6624 * t225 * t258;
    let t30643 = t214 * t30642;
    let t30645 = 0.16449340668482264365e-1 * t1880 * t30643;
    let t30646 = t8362 * t865;
    let t30647 = t2718 * t30646;
    let t30650 = t8352 * t865;
    let t30651 = t10110 * t30650;
    (t30634, t30635, t30637, t30638, t30640, t30642, t30643, t30645, t30647, t30651)
}
