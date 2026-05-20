//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta396 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1201;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1202;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta396<F: Float>(t5572: F, t9541: F, t5624: F, t9601: F, t1512: F, t47092: F, t16673: F, t2642: F, t5614: F, t9671: F, t41008: F, t5568: F, t41385: F, t5587: F, t2629: F, t2696: F, t118: F, t2375: F, t5522: F, t16710: F, t2663: F, t2517: F, t2658: F, t5392: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t58550, t58574, t58576, t58642, t58723, t58744) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1201::<F>(t5572, t9541, t5624, t9601, t1512, t47092, t16673, t2642, t5614, t9671, t41008, t5568);
        let (t58809, t58811, t58844, t58972, t58984, t59013) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1202::<F>(t41385, t5587, t16673, t2629, t2696, t118, t2375, t5522, t16710, t2663, t2517, t2658, t5392);
    (t58550, t58574, t58576, t58642, t58723, t58744, t58809, t58811, t58844, t58972, t58984, t59013)
}
