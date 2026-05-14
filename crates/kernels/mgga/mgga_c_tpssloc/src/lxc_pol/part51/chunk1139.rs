//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1139/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1139<F: Float>(t23062: F, t30700: F, t240: F, t241: F, t2627: F, t812: F, t2617: F, t30713: F, t814: F, t835: F, t30716: F, t22690: F, t23122: F, t6619: F, t776: F, t30720: F, t849: F) -> (F, F, F, F, F, F, F) {
    let t112784 = t23062 * t30700;
    let t112792 = t812 * t2627 * t240 * t241;
    let t112797 = t2617 * t30713;
    let t112802 = t812 * t814 * t835 * t241;
    let t112803 = t112802 * t30716;
    let t112804 = 7.0 / 1152.0 * t112803;
    let t112818 = t23122 * t22690 * t6619 * t776;
    let t112820 = t30720 * t849;
    (t112784, t112792, t112797, t112802, t112804, t112818, t112820)
}
