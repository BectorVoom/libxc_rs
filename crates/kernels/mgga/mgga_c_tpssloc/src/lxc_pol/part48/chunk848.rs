//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 848/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk848<F: Float>(t112723: F, t112727: F, t112730: F, t112733: F, t112742: F, t112744: F, t114772: F, t114781: F, t114785: F, t114792: F, t114795: F, t114800: F, t23214: F, t25168: F, t2713: F, t2718: F, t31409: F, t31416: F, t6662: F, t7106: F, t855: F, t8553: F, t866: F, t87013: F, t92394: F, t9593: F) -> (F,) {
    let t114802 = t112723 + 24.0 * t25168 * t92394 * t23214 + 0.3289868133696452873e-1 * t114772 - 12.0 * t87013 * t31416 + 4.0 * t855 * t2718 * t7106 * t6662 + t112727 - t112730 + t112733 - 0.82246703342411321825e-2 * t114781 + 4.0 * t9593 * t8553 - 2.0 * t114785 * t866 + 4.0 * t2713 * t31409 + t112742 + t112744 + 0.82246703342411321824e-2 * t114792 + 0.82246703342411321824e-2 * t114795 + 0.3289868133696452873e-1 * t114800;
    (t114802,)
}
