//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1957;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1958;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1959;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1960;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1961;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1962;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1963;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta537<F: Float>(t1873: F, t26114: F, t4072: F, t88: F, t6534: F, t7676: F, t2314: F, t7467: F, t5113: F, t1453: F, t22470: F, t666: F, t109: F, t22473: F, t4067: F, t6530: F, t22469: F, t22471: F, t1268: F, t1458: F, t22461: F, t24999: F, t26098: F, t26103: F, t26109: F, t26111: F, t26113: F, t6517: F, t671: F, t12725: F, t1874: F, t510: F, t652: F, t7000: F, t7685: F, t6876: F, t7688: F, t6999: F, t7753: F, t1983: F, t6880: F, t1266: F, t1976: F, t1980: F, t26002: F, t26005: F, t26006: F, t4026: F, t5361: F, t574: F, t7451: F, t7754: F, t1982: F, t8944: F, t12461: F, t2018: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26116, t26117, t26119, t26121, t26123, t26125, t26127, t26129) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1957::<F>(t1873, t26114, t4072, t88, t6534, t7676, t2314, t7467, t5113, t1453, t22470, t666);
        let t26135 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1958::<F>(t109, t22473, t26129, t4067, t6530, t22469, t22471, t26127);
        let t26138 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1959::<F>(t1268, t26135, t1458, t22461, t24999, t26098, t26103, t26109, t26111, t26113, t26116, t26119, t26121, t26123, t26125, t4072, t6517, t671);
        let (t26141, t26142, t26144, t26145, t26147, t26149, t26150) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1960::<F>(t12725, t1874, t26135, t510, t652, t7000, t7685, t6876, t7688, t6999, t7753, t1983);
        let t26155 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1961::<F>(t6880, t7685, t1266, t1976, t1980, t26002, t26005, t26006, t26098, t26138, t26141, t26144, t26145, t26147, t26150, t4026, t510, t5361, t574, t7451);
        let (t26157, t26161) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1962::<F>(t6876, t7754, t1982, t8944);
        let t26162 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1963::<F>(t12461, t2018);
    (t26117, t26129, t26135, t26138, t26142, t26149, t26155, t26157, t26161, t26162)
}
