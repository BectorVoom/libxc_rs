//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1052/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1052<F: Float>(t11644: F, t11649: F, t11719: F, t11728: F, t15446: F, t15448: F, t15450: F, t15452: F, t15503: F, t15507: F, t18297: F, t18303: F, t18307: F, t18310: F, t18312: F, t18314: F, t488: F, t4974: F, t4980: F, t4984: F, t5005: F) -> (F,) {
    let t18316 = -t11644 / 13824.0 + t11649 - t15503 * t4980 / 144.0 + t15507 * t4984 / 288.0 - t5005 * t4974 / 1152.0 - t18297 * t488 / 288.0 + t11719 * t18303 / 512.0 - t11728 * t18307 / 512.0 + t15446 - t15448 - t15450 + t15452 + t18310 / 4608.0 - t18312 / 432.0 + 19.0 / 2592.0 * t18314;
    (t18316,)
}
