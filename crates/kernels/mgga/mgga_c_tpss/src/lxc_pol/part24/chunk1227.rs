//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1227/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1227<F: Float>(t18069: F, t18079: F, t18094: F, t18098: F, t18122: F, t19847: F, t19864: F, t19880: F, t4966: F, t4970: F, t4974: F, t4980: F, t4985: F, t4991: F, t4996: F, t5001: F, t5005: F, t5009: F, t5605: F, t5610: F, t5620: F) -> (F,) {
    let t21390 = -t18079 + t19847 / 432.0 + t5605 * t4966 / 216.0 - t5605 * t4970 / 144.0 + t5605 * t4974 / 288.0 + t18094 * t4980 / 768.0 + t19864 / 1152.0 + t18069 * t4985 / 1152.0 + t5610 * t4991 / 1536.0 - t18098 * t4996 / 1536.0 - t18122 + t19880 / 1728.0 + 5.0 / 6912.0 * t5620 * t5001 - t5620 * t5005 / 1152.0 + t5620 * t5009 / 2304.0;
    (t21390,)
}
