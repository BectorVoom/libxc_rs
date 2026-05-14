//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1326/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1326<F: Float>(t20718: F, t7682: F, t18356: F, t18360: F, t18366: F, t1861: F, t19192: F, t19396: F, t19404: F, t19408: F, t20728: F, t5489: F, t5966: F, t5976: F, t5979: F, t6077: F, t63506: F, t6475: F, t65285: F, t65289: F, t65403: F) -> (F,) {
    let t68115 = t7682 * t20718;
    let t68118 = 5.0 / 3.0 * t19192 * t19404 + 5.0 / 3.0 * t19192 * t19408 + 5.0 / 6.0 * t5966 * t65285 + 5.0 / 3.0 * t5966 * t65289 + t18366 * t6475 / 3.0 + 5.0 / 6.0 * t20728 * t18360 + t65403 * t1861 / 3.0 - 5.0 / 3.0 * t63506 * t6077 + 2.0 / 3.0 * t19396 * t5976 + 5.0 / 3.0 * t20728 * t18356 + 2.0 / 3.0 * t19396 * t5979 + 5.0 / 3.0 * t68115 * t5489;
    (t68118,)
}
