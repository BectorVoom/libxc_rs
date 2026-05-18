//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 593/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk593<F: Float>(t1408: F, t1877: F, t1915: F, t25: F, t2522: F, t6670: F, t7476: F, t7541: F, t7545: F, t1409: F, t3: F, t1484: F) -> (F, F, F) {
    let t7552 = F::new(3.0) / F::new(2.0) * t2522 * t7476 + t1877 * t7541 * t25 / F::new(2.0) - t1877 * t6670 * t7545 / F::new(2.0) + t1877 * t1915 * t1408 / F::new(2.0);
    let t7573 = t3 * t1409;
    let t7634 = t1915 * t1484;
    (t7552, t7573, t7634)
}
