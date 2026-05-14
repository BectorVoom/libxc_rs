//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1155/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1155<F: Float>(t1707: F, t1710: F, t17982: F, t17984: F, t17991: F, t17993: F, t18002: F, t18006: F, t18010: F, t18014: F, t18018: F, t18024: F, t18029: F, t18034: F, t18037: F, t18040: F, t2408: F, t2426: F, t253: F, t5565: F, t5568: F, t5571: F, t5574: F, t5580: F, t5583: F, t819: F) -> (F,) {
    let t18042 = -t1707 * t18040 - t1710 * t17991 + t17982 * t253 - 2.0 * t17984 * t819 + 4.0 * t17993 * t5574 + 2.0 * t17993 * t5580 - 6.0 * t18002 * t5571 - 4.0 * t18006 * t18010 + 4.0 * t18014 * t5571 + 2.0 * t18018 * t5571 - 2.0 * t18024 * t5571 + 2.0 * t18029 * t5571 + t18034 * t5571 + t18037 * t5571 + 2.0 * t2408 * t5565 - t2426 * t5565 - 2.0 * t5568 * t5583;
    (t18042,)
}
