//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 889/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk889<F: Float>(t114552: F, t2039: F, t1873: F, t84097: F, t91854: F, t23938: F, t6534: F, t91857: F, t26977: F, t22479: F, t7042: F, t31518: F, t650: F, t2312: F, t8595: F, t22592: F, t8607: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t115796 = 2.0 * t114552 * t2039;
    let t115802 = 2.0 * t84097 * t1873;
    let t115813 = 4.0 * t91854 * t1873;
    let t115815 = 4.0 * t23938 * t6534;
    let t115817 = 2.0 * t91857 * t1873;
    let t115819 = 4.0 * t26977 * t6534;
    let t115821 = 2.0 * t7042 * t22479;
    let t115919 = 2.0 * t650 * t31518;
    let t115920 = t2312 * t8595;
    let t115922 = 6.0 * t8607 * t22592;
    (t115796, t115802, t115813, t115815, t115817, t115819, t115821, t115919, t115920, t115922)
}
