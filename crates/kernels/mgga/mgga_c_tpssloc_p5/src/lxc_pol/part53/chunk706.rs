//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 706/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk706<F: Float>(t2054: F, t259: F, t7087: F, t8539: F, t8549: F, t855: F, t8729: F, t8734: F, t8741: F) -> F {
    let t8743 = F::cast_from(0.3289868133696452873e-1_f64) * t8539 - F::cast_from(0.3289868133696452873e-1_f64) * t8549 + t8729 * t259 - F::cast_from(2.0_f64) * t7087 * t2054 + F::cast_from(2.0_f64) * t855 * t8734 - t855 * t8741;
    t8743
}
