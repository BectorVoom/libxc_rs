//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta55 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk361;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk362;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk363;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk364;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk365;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk366;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta55<F: Float>(t386: F, t68: F, t1011: F, t1014: F, t1010: F, t1022: F, t381: F, t357: F, t360: F, t1049: F, t383: F, t1003: F, t353: F, t384: F, t1050: F, t1052: F, t388: F, t991: F, t390: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1053, t1055) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk361::<F>(t386, t68);
        let t1057 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk362::<F>(t1011, t1014);
        let t1058 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk363::<F>(t1010, t1057);
        let (t1059, t1060) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk364::<F>(t1022, t381, t357, t360);
        let (t1061, t1063, t1065, t1066, t1068) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk365::<F>(t1059, t1060, t1049, t383, t1003, t1058, t353, t384, t1055, t1050, t1052, t388, t991);
        let t1070 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk366::<F>(t390);
    (t1053, t1055, t1057, t1058, t1060, t1061, t1063, t1065, t1066, t1068, t1070)
}
