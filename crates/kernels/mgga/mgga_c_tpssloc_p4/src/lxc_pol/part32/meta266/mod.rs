//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta266 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1202;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1203;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1204;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1205;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1206;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1207;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1208;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1209;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta266<F: Float>(t1202: F, t2140: F, t1209: F, sigma2: F, t1211: F, t1207: F, t1222: F, t2141: F, t1225: F, t2139: F, t471: F, t1198: F, t1218: F, t1232: F, t2134: F, t2136: F, t488: F, t7309: F, t7310: F, t7315: F, t7316: F, t7321: F, t7326: F, t7331: F, t466: F, t2145: F, t225: F, t1251: F, t2154: F, t3598: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7334, t7337) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1202::<F>(t1202, t2140, t1209, sigma2);
        let t7338 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1203::<F>(t1211, t7337);
        let t7339 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1204::<F>(t1207, t7338);
        let (t7343, t7344) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1205::<F>(t1222, t2141, t1225, t2139);
        let t7345 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1206::<F>(t471, t7344);
        let t7348 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1207::<F>(t1198, t1218, t1232, t2134, t2136, t488, t7309, t7310, t7315, t7316, t7321, t7326, t7331, t7334, t7339, t7343, t7345);
        let (t7349, t7351) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1208::<F>(t466, t7348, t2145, t225);
        let t7356 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1209::<F>(t1251, t2154, t3598);
    (t7334, t7337, t7338, t7339, t7343, t7344, t7345, t7348, t7349, t7351, t7356)
}
