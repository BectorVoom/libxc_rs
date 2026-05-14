//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1231/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1231<F: Float>(t4766: F, t5552: F, t4771: F, t5559: F, t4775: F, t17945: F, t17972: F, t20434: F, t20438: F, t20443: F, t21274: F, t21276: F, t21278: F, t21280: F, t21282: F, t1705: F, t4778: F) -> (F, F, F) {
    let t21284 = t5552 * t4766;
    let t21286 = t5559 * t4771;
    let t21288 = t5559 * t4775;
    let t21290 = t17945 + t20434 + t21274 / 16.0 - t21276 / 48.0 + t21278 / 768.0 + t20438 + t21280 / 192.0 - t21282 / 1536.0 - t21284 / 1536.0 + t17972 + t20443 + 5.0 / 384.0 * t21286 - t21288 / 384.0;
    let t21291 = param_beta * t21290;
    let t21298 = t1705 * t4778;
    (t21290, t21291, t21298)
}
