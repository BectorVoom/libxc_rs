//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1108/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1108<F: Float>(t118: F, t321: F, t5259: F, t72011: F, t76292: F, t76311: F, t76319: F, t76322: F, t78028: F, t78031: F, t78034: F, t78036: F, t78038: F, t78039: F, t78040: F, t80192: F, t80452: F) -> F {
    let t80472 = t76292 - F::new(0.39914139006212695214e-1) * t118 * t80192 - t78028 + t72011 + t78031 + t78034 + t76311 - t78036 - t78038 + t78039 + t78040 + F::new(0.11974241701863808564e0) * t5259 * t80452 * t321 + t76319 + t76322;
    t80472
}
