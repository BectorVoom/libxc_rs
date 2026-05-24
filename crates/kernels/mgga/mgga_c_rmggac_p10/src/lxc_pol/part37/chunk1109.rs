//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1109/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1109<F: Float>(t118: F, t338: F, t76326: F, t78046: F, t78048: F, t78050: F, t78051: F, t78053: F, t78055: F, t78060: F, t78061: F, t78062: F, t78065: F, t78067: F, t80372: F) -> F {
    let t80477 = F::cast_from(0.19957069503106347607e-1_f64) * t118 * t338 * t80372 + t78046 - t78048 - t78050 - t78051 + t76326 + t78053 + t78055 - t78060 + t78061 + t78062 + t78065 - t78067;
    t80477
}
