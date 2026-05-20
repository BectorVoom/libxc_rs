//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta366 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1503;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1504;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1505;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1506;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1507;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta366<F: Float>(t13546: F, t908: F, t136: F, t4389: F, t699: F, t4386: F, t10277: F, t1409: F, t2244: F, t2826: F, t4337: F, t4339: F, t690: F, t4344: F, t10564: F, t13537: F, t123: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13548, t13550, t13551, t13552, t13555) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1503::<F>(t13546, t908, t136, t4389, t699, t4386, t10277, t1409, t2244);
        let (t13557, t13559) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1504::<F>(t13555, t2826, t136, t2244, t4337);
        let (t13561, t13563) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1505::<F>(t13559, t908, t136, t4339, t690);
        let t13566 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1506::<F>(t4344, t690);
        let (t13567, t13569) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1507::<F>(t13566, t10564, t13537, t123);
    (t13548, t13550, t13551, t13552, t13555, t13557, t13559, t13561, t13563, t13566, t13567, t13569)
}
