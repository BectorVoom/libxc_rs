//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 967/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk967<F: Float>(t2018: F, t26161: F, t3698: F, t92169: F, t31338: F, t81651: F, t82074: F, t1888: F, t23270: F, t26728: F, t2719: F, t1880: F, t23196: F, t31366: F) -> (F, F, F, F) {
    let t114573 = F::new(6.0) * t26161 * t92169 * t2018 * t3698;
    let t114592 = t81651 * t82074 * t31338;
    let t114596 = t1888 * t23270 * t26728 * t2719;
    let t114599 = t1880 * t31366 * t23196;
    (t114573, t114592, t114596, t114599)
}
