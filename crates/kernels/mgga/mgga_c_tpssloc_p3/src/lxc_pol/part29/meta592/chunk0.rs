//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2018/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2018<F: Float>(t22637: F, t81228: F, t81326: F, t22638: F, t81159: F, t22892: F, t6891: F, t80645: F, t6892: F, t81186: F, t22674: F, t22934: F, t6897: F) -> (F, F, F, F, F) {
    let t81328 = t81228 * t81326 * t22637;
    let t81350 = t81159 * t22638;
    let t81365 = t22892 * t80645 * t6891;
    let t81375 = t81186 * t6892;
    let t81379 = t6897 * t22674 * t22934;
    (t81328, t81350, t81365, t81375, t81379)
}
