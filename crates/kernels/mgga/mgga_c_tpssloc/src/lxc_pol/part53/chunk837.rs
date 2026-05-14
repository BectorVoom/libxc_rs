//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 837/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk837<F: Float>(t2617: F, t30713: F, t241: F, t812: F, t814: F, t835: F, t30716: F, t22690: F, t23122: F, t6619: F, t776: F, t30720: F, t849: F, t23083: F, t30706: F, t23094: F, t30703: F) -> (F, F, F, F, F, F, F) {
    let t112797 = t2617 * t30713;
    let t112802 = t812 * t814 * t835 * t241;
    let t112803 = t112802 * t30716;
    let t112818 = t23122 * t22690 * t6619 * t776;
    let t112820 = t30720 * t849;
    let t112829 = t23083 * t30706;
    let t112834 = t23094 * t30703;
    (t112797, t112802, t112803, t112818, t112820, t112829, t112834)
}
