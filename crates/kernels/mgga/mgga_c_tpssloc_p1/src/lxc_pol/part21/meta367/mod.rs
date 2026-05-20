//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta367 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1801;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1802;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1803;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1804;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta367<F: Float>(t13559: F, t908: F, t136: F, t4339: F, t690: F, t4344: F, t10564: F, t13537: F, t123: F, t13555: F, t2768: F, t13528: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13560, t13561, t13563) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1801::<F>(t13559, t908, t136, t4339, t690);
        let t13566 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1802::<F>(t4344, t690);
        let (t13567, t13568, t13569) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1803::<F>(t13566, t10564, t13537, t123);
        let (t13571, t13572) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1804::<F>(t13555, t2768, t123);
        let (t13574, t13575) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1805::<F>(t13528, t2768, t123);
    (t13560, t13561, t13563, t13566, t13567, t13568, t13569, t13571, t13572, t13574, t13575)
}
