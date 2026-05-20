//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2175/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2175<F: Float>(t5107: F, t652: F, t6534: F, t22574: F, t56198: F, t8643: F, t26162: F, t57802: F, t22597: F, t7685: F, t2018: F, t3734: F) -> (F, F, F, F, F) {
    let t90051 = F::new(4.0) * t652 * t5107 * t6534;
    let t90059 = F::new(6.0) * t22574 * t8643 * t56198;
    let t90062 = F::new(6.0) * t22574 * t26162 * t57802;
    let t90064 = F::new(6.0) * t7685 * t22597;
    let t90065 = t3734 * t2018;
    (t90051, t90059, t90062, t90064, t90065)
}
