//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta842 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3035;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3036;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3037;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3038;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3039;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3040;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta842<F: Float>(t18242: F, t690: F, t43748: F, t50903: F, t50905: F, t50907: F, t50919: F, t50921: F, t50948: F, t50950: F, t63327: F, t63330: F, t63332: F, t63334: F, t1089: F, t55677: F, t1088: F, t123: F, t3242: F, t55723: F, t3240: F, t2394: F, t5976: F, t3247: F, t11153: F, t2244: F, t5398: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t63336 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3035::<F>(t18242, t690);
        let t63346 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3036::<F>(t43748, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t63327, t63330, t63332, t63334, t63336);
        let (t63353, t63355) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3037::<F>(t1089, t55677, t1088, t123);
        let (t63357, t63359) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3038::<F>(t3242, t55723, t123, t3240);
        let t63361 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3039::<F>(t2394, t5976);
        let (t63363, t63365) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3040::<F>(t3247, t55723, t1088, t123);
        let (t63368, t63370) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3041::<F>(t11153, t2244, t5398, t123, t3240);
    (t63336, t63346, t63353, t63355, t63357, t63359, t63361, t63363, t63365, t63368, t63370)
}
