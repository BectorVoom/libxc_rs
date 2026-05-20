//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1413;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1414;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1415;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1416;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta392<F: Float>(t1089: F, t16558: F, t1088: F, t123: F, t11137: F, t11459: F, t14702: F, t14720: F, t14946: F, t14947: F, t18203: F, t18208: F, t18213: F, t18217: F, t18219: F, t18223: F, t18227: F, t18229: F, t18234: F, t18239: F, t423: F, t14858: F, t1703: F, t4869: F, t4879: F, t1117: F, t6021: F, t3264: F, t3315: F, t6020: F, t3313: F, t4781: F, t4785: F, t11277: F, t5988: F, t11275: F, t3411: F, t6106: F, t1157: F, t6105: F, t1164: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t18241, t18243) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1413::<F>(t1089, t16558, t1088, t123);
        let t18245 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1414::<F>(t11137, t11459, t14702, t14720, t14946, t14947, t18203, t18208, t18213, t18217, t18219, t18223, t18227, t18229, t18234, t18239, t18243);
        let (t18247, t18249, t18251, t18257, t18261) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1415::<F>(t18245, t423, t14858, t1703, t4869, t4879, t1117, t6021, t3264, t3315, t6020, t3313);
        let (t18264, t18268, t18270, t18273) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1416::<F>(t4781, t4785, t3313, t11277, t5988, t1117, t11275, t3411, t6106, t1157, t6105, t1164);
    (t18241, t18243, t18247, t18249, t18251, t18257, t18261, t18264, t18268, t18270, t18273)
}
