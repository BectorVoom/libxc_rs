//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1203;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta397<F: Float>(t16616: F, t2528: F, t212: F, t5544: F, t5527: F, t5555: F, t9541: F, t41008: F, t5550: F, t16783: F, t41196: F, t16791: F, t9546: F, t2586: F, t41146: F, t9523: F, t1516: F, t47275: F, t5628: F, t9601: F, t5619: F, t9671: F, t16673: F, t2638: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t59028, t59135, t59162, t59195, t59204, t59206, t59218) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1203::<F>(t16616, t2528, t212, t5544, t5527, t5555, t9541, t41008, t5550, t16783, t41196, t16791, t9546);
        let (t59221, t59224, t59259, t59263, t59276, t59281) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1204::<F>(t2586, t41146, t59162, t59135, t9523, t1516, t47275, t5628, t9601, t5619, t9671, t16673, t2638);
    (t59028, t59195, t59204, t59206, t59218, t59221, t59224, t59259, t59263, t59276, t59281)
}
