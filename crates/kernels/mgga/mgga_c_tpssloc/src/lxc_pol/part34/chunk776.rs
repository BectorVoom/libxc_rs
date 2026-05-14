//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 776/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk776<F: Float>(t20601: F, t539: F, t1842: F, t6439: F, t12021: F, t6460: F, t3887: F, t553: F, t12249: F, t20490: F, t20495: F, t3897: F, t1380: F, t20568: F, t1825: F, t19660: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20602 = t539 * t20601;
    let t20608 = t6439 * t1842;
    let t20609 = t12021 * t20608;
    let t20612 = t1842 * t6460;
    let t20613 = t3887 * t20612;
    let t20616 = t553 * t20601;
    let t20622 = t12249 * t20490;
    let t20625 = t3897 * t20495;
    let t20630 = t1380 * t20568;
    let t20632 = t19660 * t1825;
    (t20602, t20608, t20609, t20612, t20613, t20616, t20622, t20625, t20630, t20632)
}
