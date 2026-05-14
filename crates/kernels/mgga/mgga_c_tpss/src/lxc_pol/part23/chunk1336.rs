//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1336/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1336<F: Float>(t12355: F, t12381: F, t12415: F, t12543: F, t12547: F, t19084: F, t20831: F, t20837: F, t3057: F, t3076: F, t3083: F, t3099: F, t6007: F, t63268: F, t63269: F, t63273: F, t63275: F, t63277: F, t63285: F, t63309: F, t68405: F, t68407: F, t68408: F, t68413: F, t68417: F, t68423: F, t68438: F) -> (F,) {
    let t68439 = t63309 * t12381 / 1152.0 + t68405 + t68407 + t63268 + t68408 / 1296.0 - t20831 * t3076 / 288.0 - t68413 * t3057 / 144.0 + t68417 * t3083 / 288.0 - 5.0 / 1296.0 * t20837 * t3099 + t68423 + 5.0 / 10368.0 * t63269 + t63273 / 5184.0 - t63275 / 3456.0 - t63277 / 1728.0 - t63285 / 3456.0 + t6007 * t12355 / 1536.0 + 5.0 / 6912.0 * t19084 * t12415 - t19084 * t12543 / 2304.0 - t19084 * t12547 / 1152.0 - t68438;
    (t68439,)
}
