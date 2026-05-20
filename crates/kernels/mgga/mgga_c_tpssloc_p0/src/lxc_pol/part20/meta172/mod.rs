//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta172 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1067;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta172<F: Float>(t1484: F, t213: F, t221: F, t776: F, t118: F, t794: F, t2576: F, t210: F, t214: F, t4119: F, t2562: F, t2564: F, t2569: F, t2579: F, t2590: F, t4124: F, t4127: F, t787: F, t252: F, t1492: F, t852: F, t1493: F, t225: F, t1519: F, t798: F, t1496: F, t2563: F, t1495: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4128, t4130, t4134, t4138, t4142) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1067::<F>(t1484, t213, t221, t776, t118, t794, t2576, t210, t214, t4119, t2562, t2564, t2569, t2579, t2590, t4124, t4127, t787);
        let (t4143, t4145, t4147, t4149, t4152, t4155) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1068::<F>(t252, t4142, t1492, t852, t1493, t225, t1519, t798, t1496, t2563, t1495, t210, t776);
    (t4128, t4130, t4134, t4138, t4142, t4143, t4145, t4147, t4149, t4152, t4155)
}
