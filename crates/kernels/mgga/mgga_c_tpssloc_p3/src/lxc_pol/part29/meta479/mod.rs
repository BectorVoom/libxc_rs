//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1817;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta479<F: Float>(t1089: F, t1240: F, t1251: F, t607: F, t24601: F, t225: F, t3590: F, t497: F, t462: F, t3597: F, t3599: F, t7300: F, t2123: F, t3471: F, t11613: F, t1238: F, t2121: F, t2155: F, t24564: F, t24568: F, t24571: F, t24575: F, t24577: F, t24582: F, t24587: F, t24589: F, t24591: F, t24597: F, t3487: F, t3593: F, t3600: F, t7283: F, t7351: F, t7356: F, t7392: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t24602, t24603, t24604, t24605, t24611, t24612, t24615) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1817::<F>(t1089, t1240, t1251, t607, t24601, t225, t3590, t497, t462, t3597);
        let (t24616, t24617, t24626, t24629) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1818::<F>(t24615, t3599, t7300, t2123, t3471, t11613, t1238, t2121, t2155, t24564, t24568, t24571, t24575, t24577, t24582, t24587, t24589, t24591, t24597, t24605, t24612, t3487, t3593, t3600, t7283, t7351, t7356, t7392);
    (t24602, t24603, t24604, t24605, t24611, t24612, t24615, t24616, t24617, t24626, t24629)
}
