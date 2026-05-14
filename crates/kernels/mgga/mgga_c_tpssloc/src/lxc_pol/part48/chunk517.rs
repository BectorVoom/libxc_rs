//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 517/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk517<F: Float>(t6793: F, t202: F, t6665: F, t1877: F, t1915: F, t193: F, t2522: F, t6670: F, t776: F, t868: F, t870: F, t28: F) -> (F, F, F) {
    let t6794 = 1.0 / t6793;
    let t6829 = t202 * t6665;
    let t6834 = -t1877 * t6670 * t868 + 3.0 * t1915 * t2522 * t776 + t193 * t6829 * t870;
    let t6841 = t28 * t776;
    (t6794, t6834, t6841)
}
