//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 964/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk964<F: Float>(t1009: F, t3481: F, t1011: F, t1212: F, t1174: F, t11761: F, t11766: F, t11770: F, t11774: F, t11781: F, t11787: F, t11792: F, t11794: F, t11798: F, t11802: F, t11805: F, t11809: F, t1218: F, t1227: F, t3515: F) -> (F, F, F) {
    let t11812 = t3481 * t1009;
    let t11813 = t11812 * t1011;
    let t11814 = t11813 * t1212;
    let t11817 = t1174 * t11761 / 36.0 - 7.0 / 648.0 * t1174 * t11766 - t3515 * t11770 / 1024.0 + 5.0 / 4608.0 * t1227 * t11774 - 5.0 / 5184.0 * t1227 * t11781 + 5.0 / 6912.0 * t11787 + t11792 / 6912.0 + t11794 / 768.0 - t11798 / 2304.0 - t11802 / 1152.0 - t1227 * t11805 / 4608.0 - t1227 * t11809 / 768.0 + t11814 * t1218 / 1024.0;
    (t11812, t11814, t11817)
}
