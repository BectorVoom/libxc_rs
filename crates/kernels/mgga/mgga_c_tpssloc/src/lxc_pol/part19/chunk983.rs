//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 983/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk983<F: Float>(t12120: F, t1287: F, t2225: F, t12083: F, t184: F, t17: F, t3681: F, t750: F, t1284: F, t2516: F, t521: F, t9861: F, t3826: F, t592: F, t1285: F, t2371: F, t3691: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12121 = 12.0 * t12120;
    let t12123 = 60.0 * t2225 * t1287;
    let t12124 = t12083 * t184;
    let t12125 = t17 * t12124;
    let t12126 = t3681 * t750;
    let t12127 = t17 * t12126;
    let t12128 = 3.0 * t12127;
    let t12129 = t1284 * t2516;
    let t12130 = t17 * t12129;
    let t12131 = 3.0 * t12130;
    let t12132 = t521 * t9861;
    let t12133 = t17 * t12132;
    let t12134 = t592 * t3826;
    let t12135 = 24.0 * t12134;
    let t12136 = t2225 * t1285;
    let t12137 = 60.0 * t12136;
    let t12138 = t3691 * t2371;
    (t12121, t12123, t12124, t12125, t12126, t12128, t12129, t12131, t12132, t12133, t12135, t12137, t12138)
}
