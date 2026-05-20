//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 439/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk439<F: Float>(t42: F, t2244: F, t2250: F, t43: F, t54: F, t55: F, t240: F, t59: F) -> (F, F, F, F, F, F, F) {
    let t2267 = F::new(1.0) / t42;
    let t2268 = t2267 * t2244;
    let t2271 = t43 * t2250;
    let t2274 = F::new(1.0) / t54;
    let t2275 = t2274 * t2244;
    let t2278 = t55 * t2250;
    let t2281 = t59 * t240;
    (t2267, t2268, t2271, t2274, t2275, t2278, t2281)
}
