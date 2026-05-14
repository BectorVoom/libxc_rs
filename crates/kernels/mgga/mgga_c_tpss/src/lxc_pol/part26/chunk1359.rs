//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1359/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1359<F: Float>(t15898: F, t19084: F, t1120: F, t21998: F, t15271: F, t15275: F, t15286: F, t15586: F, t15828: F, t15902: F, t20808: F, t20813: F, t6013: F, t63318: F, t68464: F, t68466: F) -> (F,) {
    let t73421 = t19084 * t15898;
    let t73440 = t21998 * t1120;
    let t73442 = -t73421 / 1728.0 - t6013 * t15828 / 576.0 - t19084 * t15902 / 2304.0 - t19084 * t15586 / 1152.0 + t68464 / 5184.0 + t63318 - t20808 * t20813 * t15286 / 144.0 - t20808 * t20813 * t15275 / 72.0 - t20808 * t20813 * t15271 / 48.0 + t68466 / 81.0 + 19.0 / 1296.0 * t73440;
    (t73442,)
}
