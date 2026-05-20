//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 748/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk748<F: Float>(t1388: F, t1390: F, t1297: F, t1307: F, t193: F, t2408: F, t2417: F, t3683: F, t3686: F, t3688: F, t3690: F, t3693: F, t3695: F, t3697: F, t3698: F, t3701: F, t3719: F, t3813: F, t3914: F, t3918: F, t533: F) -> (F, F) {
    let t3919 = t1388 * t1390;
    let t3923 = t1390 * t193 * t3914 * t533 - t193 * t3698 * t3701 * t533 + F::new(3.0) * t1297 * t193 * t3719 + F::new(6.0) * t1307 * t3918 * t3919 + t2408 + t2417 + t3683 + t3686 + t3688 - t3690 - t3693 - t3695 + t3697 + t3813;
    (t3919, t3923)
}
