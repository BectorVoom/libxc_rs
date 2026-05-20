//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta429 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1960;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1961;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta429<F: Float>(t15281: F, t4936: F, t1174: F, t3431: F, t4912: F, t1090: F, t7319: F, t4919: F, t11531: F, t11534: F, t11537: F, t11541: F, t11591: F, t15265: F, t15269: F, t15274: F, t15278: F, t3447: F, t11583: F, t3961: F) -> (F, F, F, F, F, F, F, F) {
        let (t15282, t15284, t15285, t15287, t15288, t15289, t15292) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1960::<F>(t15281, t4936, t1174, t3431, t4912, t1090, t7319, t4919, t11531, t11534, t11537, t11541, t11591, t15265, t15269, t15274, t15278, t3447);
        let t15293 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1961::<F>(t11583, t3961);
    (t15282, t15284, t15285, t15287, t15288, t15289, t15292, t15293)
}
