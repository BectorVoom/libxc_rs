//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 727/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk727<F: Float>(t120: F, t5611: F, t2639: F, t5619: F, t5614: F, t2697: F, t5628: F, t16673: F, t842: F, t5624: F, t13360: F, t1516: F, t5568: F, t9573: F, t2563: F, t5572: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16891 = t120 * t5611;
    let t16940 = t2639 * t5619;
    let t16942 = t2639 * t5614;
    let t16954 = t2697 * t5628;
    let t16976 = t16673 * t842;
    let t16988 = t2697 * t5624;
    let t16990 = t13360 * t1516;
    let t16993 = t9573 * t5568;
    let t16995 = t2563 * t5572;
    (t16891, t16940, t16942, t16954, t16976, t16988, t16990, t16993, t16995)
}
