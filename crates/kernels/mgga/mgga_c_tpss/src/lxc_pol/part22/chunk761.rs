//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 761/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk761<F: Float>(t1259: F, t4516: F, t1256: F, t1266: F, t1657: F, t3360: F, t4488: F, t4490: F, t4494: F, t538: F) -> (F, F) {
    let t4517 = t1259 * t4516;
    let t4519 = 2.0 * t1256 * t4494 - t1256 * t4517 - t1266 * t4490 - t1657 * t3360 + t4488 * t538;
    (t4517, t4519)
}
