//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1002/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1002<F: Float>(t31304: F, t6880: F, t1874: F, t84097: F, t31537: F, t7057: F, t22479: F, t89: F, t2040: F, t31540: F, t7050: F, t2314: F, t31747: F) -> (F, F, F, F, F, F) {
    let t115245 = F::new(6.0) * t31304 * t6880;
    let t115249 = F::new(2.0) * t84097 * t1874;
    let t115251 = F::new(4.0) * t31537 * t7057;
    let t115252 = t89 * t22479;
    let t115254 = F::new(2.0) * t115252 * t2040;
    let t115256 = F::new(4.0) * t31540 * t7050;
    let t115261 = F::new(4.0) * t2314 * t31747;
    (t115245, t115249, t115251, t115254, t115256, t115261)
}
