//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1001/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1001<F: Float>(t10764: F, t226: F, t773: F, t774: F, t10661: F, t10664: F, t10669: F, t10674: F, t10678: F, t10679: F, t771: F, t797: F, t8177: F, t8179: F, t8188: F, t8204: F, t8205: F, t8287: F) -> (F, F, F) {
    let t10765 = t10764 * t226;
    let t10767 = t773 * t774 * t10765;
    let t10772 = -F::new(35.0) / F::new(108.0) * t8177 + F::new(7.0) / F::new(144.0) * t8179 - t8188 - t10661 + F::new(5.0) / F::new(384.0) * t797 * t10664 + F::new(5.0) / F::new(768.0) * t797 * t10669 - F::new(5.0) / F::new(128.0) * t797 * t10674 + t10678 - F::new(119.0) / F::new(13824.0) * t10679 - t771 * t10767 / F::new(3072.0) - t8204 + F::new(7.0) / F::new(4608.0) * t8205 - F::new(119.0) / F::new(6912.0) * t8287;
    (t10765, t10767, t10772)
}
