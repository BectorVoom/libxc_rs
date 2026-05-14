//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1209/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1209<F: Float>(t32663: F, t4034: F, t1873: F, t25958: F, t652: F, t1874: F, t96361: F, t24999: F, t6525: F, t12725: F, t8323: F, t55353: F, t8319: F, t16524: F, t31280: F, t23880: F, t26550: F) -> (F, F, F, F, F, F, F, F) {
    let t120744 = t4034 * t32663;
    let t120747 = t652 * t25958 * t1873;
    let t120749 = t96361 * t1874;
    let t120751 = t24999 * t6525;
    let t120753 = t12725 * t8323;
    let t120786 = 27.0 * t55353 * t8319;
    let t120788 = 54.0 * t16524 * t31280;
    let t120789 = t23880 * t26550;
    (t120744, t120747, t120749, t120751, t120753, t120786, t120788, t120789)
}
