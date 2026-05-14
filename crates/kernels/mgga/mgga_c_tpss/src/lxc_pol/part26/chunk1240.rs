//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1240/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1240<F: Float>(t1564: F, t1575: F, t19066: F, t19084: F, t19090: F, t19094: F, t20806: F, t20821: F, t20831: F, t20834: F, t20837: F, t5215: F, t5256: F, t5262: F, t5266: F, t6002: F, t6013: F) -> (F,) {
    let t22036 = -t19090 * t5256 / 1536.0 + t6002 * t5215 / 216.0 - t19084 * t5262 / 1152.0 - t20831 * t1564 / 144.0 - t20806 / 432.0 + 5.0 / 6912.0 * t6013 * t5266 + t20837 * t1575 / 216.0 - t19066 - t19094 + t20821 / 1152.0 - t20834 / 216.0;
    (t22036,)
}
