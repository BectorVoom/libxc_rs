//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 769/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk769<F: Float>(t1268: F, t1458: F, t2039: F, t4028: F, t7042: F, t7676: F, t7787: F, t7801: F, t7170: F, t7687: F, t1807: F, t2085: F, t7181: F, t7183: F, t7185: F, t7189: F, t7706: F, t7710: F, t7713: F, t7716: F, t7718: F, t7720: F) -> (F, F, F, F) {
    let t7900 = 2.0 * t1268 * t7801 + 2.0 * t1458 * t7042 + 2.0 * t2039 * t4028 + 2.0 * t2039 * t7676 + t7787;
    let t7904 = t7170 * t7687;
    let t7910 = t1807 * t2085;
    let t7918 = -t7181 - t7706 / 24.0 - t7183 - 0.24223653656484234512e-2 * t7710 - t7185 - 0.40372756094140390853e-3 * t7713 + t7716 / 768.0 - t7718 / 768.0 - t7189 - t7720 / 192.0;
    (t7900, t7904, t7910, t7918)
}
