//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 587/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk587<F: Float>(t1408: F, t1877: F, t1915: F, t25: F, t2522: F, t6670: F, t7476: F, t7541: F, t7545: F, t1539: F, t6690: F, t6689: F) -> (F, F, F) {
    let t7552 = F::new(3.0) / F::new(2.0) * t2522 * t7476 + t1877 * t7541 * t25 / F::new(2.0) - t1877 * t6670 * t7545 / F::new(2.0) + t1877 * t1915 * t1408 / F::new(2.0);
    let t7553 = t6690 * t1539;
    let t7554 = t6689 * t7553;
    (t7552, t7553, t7554)
}
