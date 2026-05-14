//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 983/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk983<F: Float>(t1213: F, t12835: F, t12846: F, t12861: F, t12881: F, t12889: F, t13725: F, t13727: F, t13731: F, t13736: F, t13741: F, t13745: F, t13749: F, t3271: F, t4413: F, t9995: F) -> (F,) {
    let t13752 = 7.0 / 4608.0 * t13725 - 7.0 / 2304.0 * t13727 - t12835 - 119.0 / 6912.0 * t12846 - t1213 * t13731 / 48.0 - 119.0 / 3456.0 * t9995 - t4413 * t13736 / 192.0 - 35.0 / 108.0 * t12861 - t12881 - t12889 + t3271 * t13741 / 768.0 - t3271 * t13745 / 3072.0 + t3271 * t13749 / 384.0;
    (t13752,)
}
