//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2181/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2181<F: Float>(t1873: F, t55934: F, t12725: F, t6534: F, t55962: F, t19456: F, t4072: F, t649: F, t26114: F, t12813: F, t88: F, t22479: F, t4028: F) -> (F, F, F, F, F, F, F, F, F) {
    let t90363 = F::new(4.0) * t55934 * t1873;
    let t90365 = F::new(4.0) * t12725 * t6534;
    let t90367 = F::new(2.0) * t55962 * t1873;
    let t90369 = F::new(4.0) * t19456 * t6534;
    let t90370 = t649 * t4072;
    let t90372 = F::new(4.0) * t90370 * t1873;
    let t90374 = F::new(4.0) * t26114 * t6534;
    let t90375 = t88 * t12813;
    let t90377 = F::new(2.0) * t90375 * t1873;
    let t90379 = F::new(2.0) * t4028 * t22479;
    (t90363, t90365, t90367, t90369, t90370, t90372, t90374, t90377, t90379)
}
