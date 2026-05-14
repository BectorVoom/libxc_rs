//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 810/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk810<F: Float>(t5: F, t31683: F, t8308: F, t625: F, t8301: F, t2240: F, t8515: F, t1862: F, t79: F, t641: F, t8513: F, t31019: F, t31672: F, t31675: F, t31677: F, t31681: F, t8512: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t31684 = t8308 * t31683;
    let t31687 = t8301 * t625;
    let t31688 = t2240 * t31687;
    let t31690 = 5.0 / 27.0 * t31688 * t8515;
    let t31691 = t79 * t1862;
    let t31693 = t8513 * t31691 * t641;
    let t31699 = piecewise3(t8, 0.0, -5.0 / 72.0 * t31672 * t8515 + 5.0 / 12.0 * t31675 * t31677 + 5.0 / 18.0 * t31681 * t31684 + t31690 - 5.0 / 36.0 * t8512 * t31693 - 5.0 / 72.0 * t8512 * t31019);
    (t31684, t31687, t31688, t31691, t31693, t31699)
}
