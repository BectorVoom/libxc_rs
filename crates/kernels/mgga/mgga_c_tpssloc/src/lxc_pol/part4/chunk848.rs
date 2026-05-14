//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 848/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk848<F: Float>(t12998: F, t13000: F, t131: F, t9558: F, t205: F, t1489: F, t9541: F, t4126: F, t782: F, t4130: F, t2563: F, t4138: F, t4134: F, t9546: F, t118: F, t4119: F, t794: F) -> (F, F, F, F, F, F, F) {
    let t13002 = 0.49999999999999999998e-2 * t12998 * t13000;
    let t13004 = t9558 * t131;
    let t13005 = t205 * t13004;
    let t13010 = t9541 * t1489;
    let t13012 = t782 * t4126;
    let t13014 = 0.23333333333333333332e-1 * t13012 * t4130;
    let t13020 = t2563 * t4138;
    let t13022 = t9546 * t4134;
    let t13025 = t118 * t794 * t4119;
    (t13002, t13005, t13010, t13014, t13020, t13022, t13025)
}
