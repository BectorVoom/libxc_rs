//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1746;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1747;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1748;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta505<F: Float>(t1842: F, t7213: F, t3887: F, t1807: F, t7191: F, t1375: F, t16460: F, t1843: F, t2092: F, t22908: F, t22910: F, t22922: F, t22928: F, t22941: F, t24082: F, t24156: F, t24157: F, t5215: F, t5321: F, t5354: F, t568: F, t7194: F, t7199: F, t7214: F, t27005: F, t27065: F, t27127: F, t533: F, t1390: F, t671: F, t7890: F, t2075: F, t4072: F, t2039: F, t5107: F, t109: F, t26127: F, t22472: F, t23912: F, t26130: F, t26132: F, t510: F, t1458: F, t7156: F, t1983: F, t2040: F, t2314: F, t26179: F, t4028: F, t4034: F, t652: F, t7050: F, t7057: F, t7061: F, t7458: F, t7796: F, t7806: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27132, t27137, t27141) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1746::<F>(t1842, t7213, t3887, t1807, t7191, t1375, t16460, t1843, t2092, t22908, t22910, t22922, t22928, t22941, t24082, t24156, t24157, t5215, t5321, t5354, t568, t7194, t7199, t7214);
        let (t27143, t27144, t27145, t27147, t27150, t27163) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1747::<F>(t27005, t27065, t27127, t27141, t533, t1390, t671, t7890, t2075, t4072, t2039, t5107);
        let t27170 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1748::<F>(t109, t26127, t22472, t23912, t26130, t26132);
        let (t27171, t27180, t27183) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1749::<F>(t27170, t510, t1458, t7156, t1983, t2040, t2314, t26179, t27145, t27147, t27150, t27163, t4028, t4034, t652, t7050, t7057, t7061, t7458, t7796, t7806);
    (t27132, t27137, t27143, t27144, t27145, t27147, t27150, t27163, t27170, t27171, t27180, t27183)
}
