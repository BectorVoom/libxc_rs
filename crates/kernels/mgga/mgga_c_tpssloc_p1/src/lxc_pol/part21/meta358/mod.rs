//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1774;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1775;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta358<F: Float>(t120: F, t4119: F, t2645: F, t829: F, t2679: F, t4248: F, t13242: F, t4180: F, t4181: F, t4240: F, t9638: F, t2647: F, t10007: F, t4191: F, t13275: F, t13277: F, t13280: F, t13283: F, t13287: F, t13289: F, t13293: F, t13297: F, t1512: F, t2571: F, t2618: F, t2635: F, t2643: F, t2686: F, t4167: F, t4236: F, t4250: F, t9559: F, t9613: F, t9642: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13302, t13306, t13312, t13316, t13320, t13322) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1774::<F>(t120, t4119, t2645, t829, t2679, t4248, t13242, t4180, t4181, t4240, t9638, t2647);
        let (t13326, t13330, t13331) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1775::<F>(t10007, t2645, t4181, t4191, t9638, t13275, t13277, t13280, t13283, t13287, t13289, t13293, t13297, t13302, t13306, t13312, t13316, t13320, t13322, t1512, t2571, t2618, t2635, t2643, t2686, t4167, t4236, t4250, t9559, t9613, t9642);
    (t13302, t13306, t13312, t13316, t13320, t13322, t13326, t13330, t13331)
}
