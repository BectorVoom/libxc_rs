//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta728 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2365;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2366;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2367;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2368;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta728<F: Float>(t100908: F, t100915: F, t100917: F, t100921: F, t100924: F, t100927: F, t100929: F, t100932: F, t100934: F, t100936: F, t100938: F, t100941: F, t1458: F, t20176: F, t20181: F, t24972: F, t27921: F, t4072: F, t5376: F, t5456: F, t85416: F, t96311: F, t96334: F, t1851: F, t8119: F, t103103: F, t105102: F, t105115: F, t1396: F, t1398: F, t1404: F, t1852: F, t20149: F, t2174: F, t27930: F, t29866: F, t29884: F, t3: F, t5364: F, t580: F, t6483: F, t7416: F, t96281: F, t96283: F, t96285: F, t1858: F, t8110: F, t29865: F, t2169: F, t576: F, t20186: F, t2170: F, t27908: F, t5381: F, t6471: F, t7426: F, t8111: F, t96289: F, t96291: F, t96300: F, t96303: F, t96308: F) -> F {
        let t105128 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2365::<F>(t100908, t100915, t100917, t100921, t100924, t100927, t100929, t100932, t100934, t100936, t100938, t100941, t1458, t20176, t20181, t24972, t27921, t4072, t5376, t5456, t85416, t96311, t96334);
        let t105139 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2366::<F>(t1851, t8119, t103103, t105102, t105115, t105128, t1396, t1398, t1404, t1852, t20149, t2174, t27930, t29866, t29884, t3, t5364, t580, t6483, t7416, t96281, t96283, t96285);
        let t105151 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2367::<F>(t1858, t8110, t29865, t580, t2169, t6483, t29884, t576, t20186, t2170, t27908, t5381, t6471, t7426, t8111, t96289, t96291, t96300, t96303, t96308);
        let tv4rho3sigma8 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2368::<F>(t105139, t105151);
    tv4rho3sigma8
}
