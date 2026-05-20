//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2357/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2357<F: Float>(t104990: F, t1459: F, t1774: F, t19461: F, t19534: F, t2165: F, t27290: F, t27293: F, t27371: F, t4028: F, t5457: F, t652: F, t672: F, t7408: F, t7458: F, t96238: F, t96833: F, t96837: F, t96839: F, t96842: F, t96844: F, t96846: F, t97777: F, t97779: F, t97783: F, t97785: F, t97788: F) -> F {
    let t105024 = -F::new(2.0) * t19534 * t2165 * t652 - F::new(2.0) * t104990 * t672 - F::new(4.0) * t1459 * t96238 - F::new(2.0) * t1774 * t27371 - F::new(2.0) * t19461 * t2165 - F::new(4.0) * t27290 * t7458 - F::new(4.0) * t27293 * t4028 - F::new(2.0) * t5457 * t7408 + t96833 - t96837 - t96839 - t96842 - t96844 - t96846 + t97777 - t97779 - t97783 - t97785 - t97788;
    t105024
}
