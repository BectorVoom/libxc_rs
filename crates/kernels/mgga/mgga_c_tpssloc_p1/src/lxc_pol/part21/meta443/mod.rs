//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1987;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1988;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1989;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1990;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta443<F: Float>(t11697: F, t4949: F, t3577: F, t3431: F, t4729: F, t1174: F, t1177: F, t14749: F, t14753: F, t14744: F, t1011: F, t15031: F, t1212: F, t1226: F, t4965: F, t11652: F, t11665: F, t11678: F, t11692: F, t11699: F, t11703: F, t1218: F, t1232: F, t15560: F, t15564: F, t15569: F, t3496: F, t3580: F, t4950: F, t5002: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15572, t15574, t15578, t15580, t15581, t15584, t15587, t15590) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1987::<F>(t11697, t4949, t3577, t3431, t4729, t1174, t1177, t14749, t14753, t14744, t1011, t15031);
        let t15591 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1988::<F>(t1212, t15590);
        let t15594 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1989::<F>(t1226, t4965);
        let t15601 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1990::<F>(t11652, t11665, t11678, t11692, t11699, t11703, t1174, t1218, t1232, t15560, t15564, t15569, t15574, t15580, t15581, t15584, t15587, t15591, t15594, t3496, t3580, t4950, t5002);
    (t15572, t15574, t15578, t15580, t15581, t15584, t15587, t15590, t15591, t15594, t15601)
}
