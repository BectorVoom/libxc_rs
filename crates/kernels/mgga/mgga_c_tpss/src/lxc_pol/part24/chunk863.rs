//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 863/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk863<F: Float>(t5640: F, t6179: F, t1731: F, t347: F, t6167: F, t1483: F, t1730: F, t1733: F, t373: F, t5626: F, t5631: F, t5639: F, t6168: F, t6172: F, t6175: F, t1364: F, t1713: F) -> (F, F, F, F) {
    let t6180 = t5640 * t6179;
    let t6183 = t1731 * t347 * t6167;
    let t6185 = -t1483 * t5626 - t1730 * t6183 - t1733 * t6172 + t373 * t6168 + 2.0 * t5631 * t6175 - t5639 * t6180;
    let t6192 = t1713 * t1364;
    (t6180, t6183, t6185, t6192)
}
