//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1897;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta634<F: Float>(t22779: F, t28060: F, t19661: F, t1992: F, t22897: F, t19736: F, t22892: F, t22893: F, t28138: F, t28116: F, t81228: F, t81326: F, t6897: F, t7700: F, t90544: F, t214: F, t6434: F, t1985: F, t6907: F, t22633: F, t26215: F, t90566: F, t22635: F, t26354: F, t5353: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t97463, t97488, t97491, t97494, t97503) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1897::<F>(t22779, t28060, t19661, t1992, t22897, t19736, t22892, t22893, t28138, t28116, t81228, t81326);
        let (t97509, t97511, t97513, t97516, t97524) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1898::<F>(t6897, t7700, t90544, t214, t6434, t1985, t6907, t22633, t26215, t90566, t1992, t22635, t26354, t5353);
    (t97463, t97488, t97491, t97494, t97503, t97509, t97511, t97513, t97516, t97524)
}
