//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1068/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1068<F: Float>(t2225: F, t3696: F, t12124: F, t588: F, t592: F, t1287: F, t9212: F, t1285: F, t12083: F, t17: F, t750: F, t2516: F, t3681: F, t12126: F, t3914: F, t9218: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t39628 = t2225 * t3696;
    let t39629 = 120.0 * t39628;
    let t39630 = t588 * t12124;
    let t39631 = 16.0 * t39630;
    let t39632 = t592 * t12124;
    let t39633 = 16.0 * t39632;
    let t39634 = t9212 * t1287;
    let t39635 = 96.0 * t39634;
    let t39636 = t9212 * t1285;
    let t39637 = 96.0 * t39636;
    let t39639 = t17 * t12083 * t750;
    let t39640 = 4.0 * t39639;
    let t39642 = t17 * t3681 * t2516;
    let t39643 = 6.0 * t39642;
    let t39644 = t592 * t12126;
    let t39645 = 48.0 * t39644;
    let t39649 = t3914 * t3914;
    let t39655 = 480.0 * t9218 * t1287;
    (t39629, t39631, t39633, t39635, t39637, t39640, t39643, t39645, t39649, t39655)
}
