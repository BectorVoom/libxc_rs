//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1557;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1558;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta349<F: Float>(t16891: F, t4180: F, t4182: F, t120: F, t5527: F, t829: F, t9646: F, t5544: F, t2645: F, t16839: F, t2647: F, t13177: F, t13251: F, t13260: F, t13275: F, t13277: F, t13280: F, t13287: F, t13320: F, t13330: F, t1512: F, t16872: F, t16877: F, t16879: F, t16888: F, t2643: F, t4167: F, t4178: F, t4191: F, t4236: F, t4240: F, t4250: F, t831: F, t232: F, t4119: F, t4181: F, t13242: F, t5591: F, t13228: F, t13351: F, t13222: F, t9627: F, t2632: F, t4233: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16893, t16898, t16903, t16907, t16910) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1557::<F>(t16891, t4180, t4182, t120, t5527, t829, t9646, t5544, t2645, t16839, t2647, t13177, t13251, t13260, t13275, t13277, t13280, t13287, t13320, t13330, t1512, t16872, t16877, t16879, t16888, t2643, t4167, t4178, t4191, t4236, t4240, t4250, t831);
        let (t16912, t16914, t16918, t16924, t16928, t16932) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1558::<F>(t232, t4119, t2645, t4181, t16891, t2647, t13242, t5591, t13228, t13351, t13222, t16839, t9627);
        let t16935 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1559::<F>(t2632, t4233);
    (t16893, t16898, t16903, t16907, t16910, t16912, t16914, t16918, t16924, t16928, t16932, t16935)
}
