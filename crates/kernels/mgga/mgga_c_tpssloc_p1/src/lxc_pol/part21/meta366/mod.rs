//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1798;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1799;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1800;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta366<F: Float>(t13546: F, t908: F, t136: F, t4389: F, t699: F, t4386: F, t10277: F, t1409: F, t2244: F, t2826: F, t4337: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13547, t13548, t13550, t13551, t13552) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1798::<F>(t13546, t908, t136, t4389, t699, t4386);
        let (t13554, t13555) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1799::<F>(t10277, t1409, t2244);
        let (t13556, t13557, t13559) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1800::<F>(t13555, t2826, t136, t2244, t4337);
    (t13547, t13548, t13550, t13551, t13552, t13554, t13555, t13556, t13557, t13559)
}
