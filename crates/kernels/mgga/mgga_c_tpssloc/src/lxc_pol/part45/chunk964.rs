//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 964/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk964<F: Float>(t31286: F, t23893: F, t24465: F, t23896: F, t55571: F, t8657: F, t1873: F, t23917: F, t3941: F, t6534: F, t7056: F, t45560: F) -> (F, F, F, F, F, F, F) {
    let t114500 = F::new(54.0) * t31286;
    let t114513 = F::new(54.0) * t24465 * t23893;
    let t114515 = F::new(27.0) * t24465 * t23896;
    let t114517 = F::new(27.0) * t55571 * t8657;
    let t114520 = F::new(27.0) * t3941 * t23917 * t1873;
    let t114525 = F::new(54.0) * t3941 * t7056 * t6534;
    let t114527 = F::new(27.0) * t45560 * t8657;
    (t114500, t114513, t114515, t114517, t114520, t114525, t114527)
}
