//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta659 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2201;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2202;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta659<F: Float>(t13261: F, t4166: F, t118: F, t2375: F, t5522: F, t16575: F, t706: F, t16710: F, t2663: F, t157: F, t46387: F, t12939: F, t5392: F, t607: F, t750: F, t2517: F, t2658: F, t12923: F, t3966: F, t4194: F, t12924: F, t16693: F, t16616: F, t2528: F, t12932: F, t4205: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t58904, t58972, t58976, t58984, t58994, t59004) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2201::<F>(t13261, t4166, t118, t2375, t5522, t16575, t706, t16710, t2663, t157, t46387, t12939, t5392, t607, t750);
        let (t59013, t59022, t59024, t59028, t59032) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2202::<F>(t2517, t2658, t5392, t12923, t3966, t4194, t12924, t16693, t16616, t2528, t12932, t4205);
    (t58904, t58972, t58976, t58984, t58994, t59004, t59013, t59022, t59024, t59028, t59032)
}
