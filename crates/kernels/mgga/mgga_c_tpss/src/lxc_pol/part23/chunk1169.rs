//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1169/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1169<F: Float>(t1877: F, t3087: F, t3092: F, t6013: F, t19066: F, t19067: F, t19077: F, t19080: F, t19084: F, t19090: F, t3035: F, t3040: F, t3044: F, t3057: F, t3070: F, t3076: F, t3083: F, t3099: F, t3103: F, t3107: F, t6002: F, t6007: F) -> (F, F, F) {
    let t19094 = t1877 * t3087 / 6912.0;
    let t19095 = t6013 * t3092;
    let t19103 = -t19066 - t19067 / 432.0 + t6002 * t3035 / 216.0 - t6002 * t3040 / 144.0 - t6002 * t3044 / 288.0 + t19077 * t3057 / 768.0 + t19080 / 1152.0 - t19084 * t3070 / 1152.0 + t6007 * t3076 / 1536.0 - t19090 * t3083 / 1536.0 - t19094 - t19095 / 1728.0 + 5.0 / 6912.0 * t6013 * t3099 - t6013 * t3103 / 1152.0 - t6013 * t3107 / 2304.0;
    (t19094, t19095, t19103)
}
