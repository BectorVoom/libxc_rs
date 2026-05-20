//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1195;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1196;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta391<F: Float>(t19681: F, t2535: F, t2371: F, t19575: F, t592: F, t2221: F, t6328: F, t2223: F, t2225: F, t17: F, t2516: F, t6320: F, t212: F, t6330: F, t2586: F, t40353: F, t6347: F, t12225: F, t40018: F, t6353: F, t12189: F, t6358: F, t19767: F, t40409: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t56104, t56168, t56185, t56390, t56392, t56394, t56398) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1195::<F>(t19681, t2535, t2371, t19575, t592, t2221, t6328, t2223, t2225, t17, t2516, t6320);
        let (t56465, t56469, t56484, t56491, t56535) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1196::<F>(t212, t6330, t2586, t40353, t6347, t12225, t40018, t6353, t12189, t6358, t19767, t40409);
    (t56104, t56168, t56185, t56390, t56392, t56394, t56398, t56465, t56469, t56484, t56491, t56535)
}
