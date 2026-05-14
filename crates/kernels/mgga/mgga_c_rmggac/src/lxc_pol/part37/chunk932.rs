//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 932/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk932<F: Float>(t70195: F, t70198: F, t70212: F, t73448: F, t73454: F, t78394: F, t78395: F, t78397: F, t78399: F, t78400: F, t78401: F, t78402: F, t78403: F, t78404: F, t78405: F, t78406: F, t78409: F) -> (F,) {
    let t80388 = -t78394 + t78395 + t78397 + t78399 - t78400 - t78401 + t73448 - t78402 - t78403 + t78404 - t78405 + t78406 - t73454 + t70195 + 0.8283415761659696377e-1 * t70198 - t78409 + t70212;
    (t80388,)
}
