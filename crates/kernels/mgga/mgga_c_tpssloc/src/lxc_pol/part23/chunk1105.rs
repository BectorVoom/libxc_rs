//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1105/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1105<F: Float>(t19815: F, t3798: F, t1827: F, t54532: F, t19575: F, t588: F, t19541: F, t2663: F, t118: F, t2375: F, t6320: F, t12365: F, t6422: F, t3862: F, t6379: F, t3787: F, t6434: F) -> (F, F, F, F, F, F, F, F) {
    let t57056 = t19815 * t3798;
    let t57073 = t54532 * t1827;
    let t57208 = t588 * t19575;
    let t57211 = t19541 * t2663;
    let t57235 = t6320 * t118 * t2375;
    let t57310 = t12365 * t6422;
    let t57383 = t6379 * t3862;
    let t57653 = t3787 * t6434;
    (t57056, t57073, t57208, t57211, t57235, t57310, t57383, t57653)
}
