//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 996/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk996<F: Float>(t1233: F, t13698: F, t4415: F, t12863: F, t3273: F, t5387: F, t1213: F, t12835: F, t12846: F, t12861: F, t12881: F, t12889: F, t13725: F, t13727: F, t13731: F, t13736: F, t13741: F, t3271: F, t4413: F, t9995: F) -> (F, F, F) {
    let t13745 = t4415 * t13698 * t1233;
    let t13749 = t3273 * t12863 * t5387;
    let t13752 = F::new(7.0) / F::new(4608.0) * t13725 - F::new(7.0) / F::new(2304.0) * t13727 - t12835 - F::new(119.0) / F::new(6912.0) * t12846 - t1213 * t13731 / F::new(48.0) - F::new(119.0) / F::new(3456.0) * t9995 - t4413 * t13736 / F::new(192.0) - F::new(35.0) / F::new(108.0) * t12861 - t12881 - t12889 + t3271 * t13741 / F::new(768.0) - t3271 * t13745 / F::new(3072.0) + t3271 * t13749 / F::new(384.0);
    (t13745, t13749, t13752)
}
