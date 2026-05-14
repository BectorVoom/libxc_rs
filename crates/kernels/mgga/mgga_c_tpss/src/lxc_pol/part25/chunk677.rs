//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 677/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk677<F: Float>(t1196: F, t1270: F, t198: F, t2281: F, t2285: F, t3182: F, t3189: F, t3194: F, t3196: F, t4357: F, t4359: F, t4379: F, t4397: F, t4428: F, t4429: F, t4431: F, t4433: F, t4437: F, t4519: F, t509: F) -> (F,) {
    let t4523 = t1270 * t198 * t4519 * t509 + 3.0 * t1196 * t198 * t4397 - t2281 - t2285 - t3182 + t3189 + t3194 - t3196 + t4357 - t4359 + t4379 - t4428 - t4429 + t4431 + t4433 - t4437;
    (t4523,)
}
