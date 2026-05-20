//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 985/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk985<F: Float>(t109: F, t22473: F, t26129: F, t4067: F, t6530: F, t22469: F, t22471: F, t26127: F) -> F {
    let t110 = F::new(1.0) < t109;
    let t26130 = t22473 * t26129;
    let t26132 = t6530 * t4067;
    let t26135 = piecewise3::<F>(t110, F::new(0.0), t22469 + t22471 / F::new(3.0) + t26127 / F::new(3.0) + t26130 / F::new(4.0) - t26132 / F::new(8.0));
    t26135
}
