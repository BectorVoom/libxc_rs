//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 441/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk441<F: Float>(t42: F, t54: F, t240: F, t59: F, t40: F, t632: F, t73: F, t52: F, t636: F, t76: F, t111: F, t649: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2267 = 1.0 / t42;
    let t2274 = 1.0 / t54;
    let t2281 = t59 * t240;
    let t2282 = 88.0 / 9.0 * t2281;
    let t2289 = t632 * t40;
    let t2291 = 1.0 / t73 / t2289;
    let t2296 = t636 * t52;
    let t2298 = 1.0 / t76 / t2296;
    let t2314 = t649 * t111;
    (t2267, t2274, t2281, t2282, t2289, t2291, t2296, t2298, t2314)
}
