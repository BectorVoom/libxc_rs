//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 8/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk8<F: Float>(t14: F, t9: F, t10: F, t15: F, t11: F, t17: F) -> (F, F, F, F) {
    let t19 = t14 * t9;
    let t20 = t15 * t10;
    let t21 = 1.0 / t20;
    let t24 = 0.35e0 + 0.87e0 * t9 * t11 + 0.5e0 * t17 + 0.226e1 * t19 * t21;
    (t19, t20, t21, t24)
}
