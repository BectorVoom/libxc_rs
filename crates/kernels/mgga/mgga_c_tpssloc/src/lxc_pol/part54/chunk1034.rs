//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1034/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1034<F: Float>(t232: F, t2646: F, t4180: F, t30714: F, t235: F, t835: F, t226: F, t8344: F, t8343: F, t849: F, t6547: F, t8336: F, t25: F, t6665: F, t28: F, t3701: F, t6995: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30716 = t4180 * t2646 * t232;
    let t30717 = t30714 * t30716;
    let t30719 = t235 * t835;
    let t30720 = t226 * t30719;
    let t30721 = t30720 * t8344;
    let t30723 = t8343 * t849;
    let t30748 = 0.38381794893125283518e-1 * t6547 * t8336;
    let t30767 = t25 * t6665;
    let t30974 = t28 * t6665;
    let t31035 = t3701 * t6995;
    (t30716, t30717, t30719, t30720, t30721, t30723, t30748, t30767, t30974, t31035)
}
