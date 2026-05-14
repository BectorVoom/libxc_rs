//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1225/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1225<F: Float>(t19724: F, t219: F, t6131: F, t1705: F, t3692: F, t935: F, t5570: F, t6134: F, t6137: F, t818: F, t18000: F, t1395: F, t226: F, t782: F, t18007: F, t1396: F, t1710: F, t17984: F, t17993: F, t18006: F, t253: F, t3699: F, t3722: F, t5565: F, t5571: F, t5574: F, t5580: F, t5583: F, t6135: F, t6138: F, t819: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19725 = param_beta * t19724;
    let t19727 = t6131 * t219;
    let t19733 = t1705 * t3692;
    let t19734 = t19733 * t935;
    let t19736 = t6134 * t5570;
    let t19743 = t6137 * t818;
    let t19744 = t18000 * t19743;
    let t19748 = t1395 * t782 * t226;
    let t19749 = t18007 * t19748;
    let t19752 = -t1396 * t17984 - t1710 * t19734 + 2.0 * t17993 * t6138 - 2.0 * t18006 * t19749 + t19725 * t253 - t19727 * t819 + 2.0 * t19736 * t5574 + t19736 * t5580 - 6.0 * t19744 * t5571 + 2.0 * t3699 * t5565 - t3722 * t5565 - t5583 * t6135;
    (t19725, t19727, t19733, t19734, t19736, t19744, t19748, t19749, t19752)
}
