//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1604;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta392<F: Float>(t1147: F, t1156: F, t14829: F, t1164: F, t3423: F, t4869: F, t11126: F, t1703: F, t1657: F, t3263: F, t3266: F, t11292: F, t1694: F, t3404: F, t1098: F, t4737: F, t1119: F, t3308: F, t4740: F, t3312: F, t3316: F, t11282: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14833, t14835, t14837, t14840, t14841) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1604::<F>(t1147, t1156, t14829, t1164, t3423, t4869, t11126, t1703, t1657, t3263, t3266, t11292, t1694);
        let (t14844, t14847, t14849, t14852, t14853) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1605::<F>(t14841, t3404, t1164, t1098, t4737, t1119, t3308, t4740, t1657, t3312, t3316, t11282, t1694);
    (t14833, t14835, t14837, t14840, t14844, t14847, t14849, t14852, t14853)
}
