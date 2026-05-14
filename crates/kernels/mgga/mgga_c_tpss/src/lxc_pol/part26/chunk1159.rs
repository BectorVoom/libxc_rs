//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1159/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1159<F: Float>(t1877: F, t3087: F, t3092: F, t6013: F, t219: F, t6017: F, t5570: F, t6021: F, t6030: F, t452: F, t9738: F, t3117: F, t5637: F, t1883: F) -> (F, F, F, F, F, F, F, F) {
    let t19094 = t1877 * t3087 / 6912.0;
    let t19095 = t6013 * t3092;
    let t19106 = t6017 * t219;
    let t19115 = t6021 * t5570;
    let t19118 = t6021 * t6030;
    let t19123 = t9738 * t452;
    let t19128 = t5637 * t3117;
    let t19129 = t1883 * t19128;
    (t19094, t19095, t19106, t19115, t19118, t19123, t19128, t19129)
}
