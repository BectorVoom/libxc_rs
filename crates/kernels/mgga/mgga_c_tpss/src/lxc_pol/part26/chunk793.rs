//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 793/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk793<F: Float>(t1259: F, t5448: F, t1256: F, t1657: F, t4490: F, t538: F, t5428: F, t5433: F) -> (F, F) {
    let t5449 = t1259 * t5448;
    let t5451 = 2.0 * t1256 * t5433 - t1256 * t5449 - 2.0 * t1657 * t4490 + t538 * t5428;
    (t5449, t5451)
}
