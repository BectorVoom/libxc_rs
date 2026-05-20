//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1810/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1810<F: Float>(t23030: F, t25205: F, t23164: F, t7479: F, t82133: F, t23204: F, t25216: F, t6562: F, t1519: F, t212: F, t23171: F, t6554: F) -> (F, F, F, F) {
    let t87898 = t23030 * t25205;
    let t87901 = t23164 * t82133 * t7479;
    let t87910 = t6562 * t23204 * t25216;
    let t87915 = t23171 * t212 * t1519 * t6554;
    (t87898, t87901, t87910, t87915)
}
