//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 701/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk701<F: Float>(t3034: F, t334: F, t371: F, t202: F, t6665: F, t1877: F, t1915: F, t193: F, t2522: F, t6670: F, t776: F, t868: F, t870: F) -> (F, F, F) {
    let t6739 = F::new(1.0) / t3034 / t334;
    let t6793 = t371 * t334;
    let t6794 = F::new(1.0) / t6793;
    let t6829 = t202 * t6665;
    let t6834 = -t1877 * t6670 * t868 + F::new(3.0) * t1915 * t2522 * t776 + t193 * t6829 * t870;
    (t6739, t6794, t6834)
}
