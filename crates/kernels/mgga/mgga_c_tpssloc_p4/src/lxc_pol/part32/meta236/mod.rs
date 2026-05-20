//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta236 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1065;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1066;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1067;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1068;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1069;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta236<F: Float>(t466: F, t6238: F, t1760: F, t3598: F, t491: F, t6224: F, t3612: F, t1734: F, t1751: F, t1246: F, t6218: F, t3625: F, t493: F, t1244: F, t1729: F, t1756: F, t1758: F, t3610: F, t3624: F, t470: F, t494: F, t5064: F, t6168: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6239, t6243) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1065::<F>(t466, t6238, t1760);
        let t6244 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1066::<F>(t3598, t6243);
        let t6252 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1067::<F>(t491, t6224);
        let (t6253, t6256) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1068::<F>(t3612, t6252, t1734, t1751);
        let (t6257, t6260) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1069::<F>(t1246, t6256, t491, t6218);
        let (t6261, t6263, t6265, t6267) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1070::<F>(t1246, t6260, t3625, t6252, t493, t6238, t1244, t1729, t1756, t1758, t3610, t3624, t470, t494, t5064, t6168, t6253, t6257);
    (t6239, t6243, t6244, t6252, t6253, t6256, t6257, t6260, t6261, t6263, t6265, t6267)
}
