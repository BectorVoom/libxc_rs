//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1038/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1038<F: Float>(t22751: F, t6970: F, t3853: F, t6945: F, t3777: F, t6944: F, t1354: F, t3787: F, t59: F, t240: F, t1336: F, t3795: F, t6943: F, t835: F, t3858: F, t1339: F, t3851: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t22752 = t22751 * t6970;
    let t22753 = 0.76763589786250567036e-1 * t22752;
    let t22754 = t6945 * t3853;
    let t22756 = t3777 * t6944;
    let t22757 = t22756 * t1354;
    let t22759 = t3787 * t59;
    let t22760 = t22759 * t240;
    let t22761 = t1336 * t22760;
    let t22762 = t22761 * t3795;
    let t22764 = t6943 * t835;
    let t22765 = t1336 * t22764;
    let t22766 = t22765 * t1354;
    let t22767 = 7.0 / 1152.0 * t22766;
    let t22768 = t6945 * t3858;
    let t22770 = t1339 * t3851;
    (t22753, t22754, t22756, t22757, t22759, t22760, t22761, t22762, t22764, t22765, t22767, t22768, t22770)
}
