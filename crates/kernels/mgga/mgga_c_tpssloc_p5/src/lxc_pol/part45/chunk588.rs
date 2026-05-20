//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 588/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk588<F: Float>(t1958: F, t3216: F, t202: F, t6665: F, t1877: F, t1915: F, t193: F, t2522: F, t6670: F, t776: F, t868: F, t870: F) -> (F, F) {
    let t6822 = t1958 * t3216;
    let t6829 = t202 * t6665;
    let t6834 = -t1877 * t6670 * t868 + F::new(3.0) * t1915 * t2522 * t776 + t193 * t6829 * t870;
    (t6822, t6834)
}
