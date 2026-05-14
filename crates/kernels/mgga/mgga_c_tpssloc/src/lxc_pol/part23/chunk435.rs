//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 435/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk435<F: Float>(t2229: F, t19: F, t84: F, t85: F, t24: F, t42: F, t54: F, t240: F, t59: F) -> (F, F, F, F, F, F, F) {
    let t2230 = 1.0 / t2229;
    let t2232 = 0.9492e2 * t19 * t2230;
    let t2239 = 1.0 / t85 / t84;
    let t2240 = t24 * t2239;
    let t2267 = 1.0 / t42;
    let t2274 = 1.0 / t54;
    let t2281 = t59 * t240;
    (t2230, t2232, t2239, t2240, t2267, t2274, t2281)
}
