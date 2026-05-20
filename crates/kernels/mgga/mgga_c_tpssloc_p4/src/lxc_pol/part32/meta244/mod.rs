//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta244 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1104;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1105;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1106;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1107;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1108;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1109;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta244<F: Float>(t652: F, t6525: F, t107: F, t625: F, t63: F, t656: F, t109: F, t666: F, t510: F, t25: F, t776: F, t154: F, t781: F, t1879: F, t1883: F, t131: F, t209: F, t229: F, t1878: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6527, t6529, t6530) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1104::<F>(t652, t6525, t107, t625, t63, t656);
        let t6534 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1105::<F>(t109, t6530, t666, t6529);
        let t6535 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1106::<F>(t510, t6534);
        let (t6537, t6542, t6546) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1107::<F>(t652, t6535, t25, t776, t154, t781);
        let t6547 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1108::<F>(t1879, t6546);
        let (t6549, t6551, t6552) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1109::<F>(t1883, t6547, t131, t209, t229, t1878);
    (t6527, t6529, t6530, t6534, t6535, t6537, t6542, t6546, t6547, t6549, t6551, t6552)
}
