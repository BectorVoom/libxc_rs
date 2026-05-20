//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1752;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta467<F: Float>(t1015: F, t6746: F, t23472: F, t40: F, t984: F, t1933: F, t225: F, t343: F, t364: F, t6721: F, t6739: F, t6741: F, t344: F, t6729: F, t6740: F, t3103: F, t6755: F, t3034: F, t371: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23473, t23474, t23476, t23477, t23478, t23479) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1752::<F>(t1015, t6746, t23472, t40, t984, t1933, t225, t343, t364);
        let (t23480, t23482, t23483, t23488, t23489, t23500, t23508) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1753::<F>(t23477, t23479, t6721, t6739, t6741, t344, t6729, t6740, t3103, t6755, t3034, t371);
    (t23473, t23474, t23476, t23478, t23479, t23480, t23482, t23483, t23488, t23489, t23500, t23508)
}
