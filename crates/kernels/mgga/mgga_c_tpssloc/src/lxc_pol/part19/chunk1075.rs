//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1075/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1075<F: Float>(t25: F, t12129: F, t592: F, t17: F, t184: F, t39454: F, t1287: F, t9216: F, t2223: F, t3826: F, t11985: F, t514: F, t11987: F, t11991: F, t1298: F, t2249: F, t3665: F, t3704: F, t39109: F, t39420: F, t39426: F, t9257: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t39851 = t592 * t12129;
    let t39852 = 48.0 * t39851;
    let t39854 = t17 * t39454 * t184;
    let t39855 = t9216 * t1287;
    let t39856 = 960.0 * t39855;
    let t39857 = t2223 * t3826;
    let t39858 = 384.0 * t39857;
    let t39861 = 1.0 / t514 / t11985 / t25;
    let t39874 = piecewise3(t26, 0.0, -56.0 / 81.0 * t39861 * t39420 + 16.0 / 9.0 * t11987 * t3665 * t2249 - 2.0 / 3.0 * t3704 * t39426 - 8.0 / 9.0 * t11991 * t9257 + 2.0 / 3.0 * t1298 * t39109);
    (t39852, t39854, t39856, t39858, t39874)
}
