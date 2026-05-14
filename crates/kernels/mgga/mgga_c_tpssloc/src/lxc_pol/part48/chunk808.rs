//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 808/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk808<F: Float>(t112797: F, t30716: F, t241: F, t812: F, t814: F, t835: F, t232: F, t30714: F, t4180: F, t9626: F, t9621: F, t23046: F, t2633: F, t6605: F, t22690: F, t23122: F, t6619: F, t776: F) -> (F, F, F, F, F, F) {
    let t112798 = t112797 * t30716;
    let t112802 = t812 * t814 * t835 * t241;
    let t112803 = t112802 * t30716;
    let t112807 = t30714 * t4180 * t9626 * t232;
    let t112811 = t30714 * t4180 * t9621 * t232;
    let t112814 = t6605 * t23046 * t2633;
    let t112818 = t23122 * t22690 * t6619 * t776;
    (t112798, t112803, t112807, t112811, t112814, t112818)
}
