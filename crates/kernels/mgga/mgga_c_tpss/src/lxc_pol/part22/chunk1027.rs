//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1027/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1027<F: Float>(t11869: F, t4223: F, t4241: F, t9561: F, t3067: F, t1114: F, t4056: F, t3068: F, t1501: F, t3074: F, t1098: F, t12279: F, t12290: F, t12294: F, t12295: F, t12298: F, t12301: F, t12304: F, t12307: F, t3103: F, t3107: F, t4265: F, t9526: F, t9530: F, t9535: F, t9538: F, t9543: F, t9547: F) -> (F,) {
    let t12310 = t4223 * t11869;
    let t12317 = t9561 * t4241;
    let t12319 = t3067 * t12317 / 3456.0;
    let t12320 = t4056 * t1114;
    let t12321 = t3068 * t12320;
    let t12324 = t1501 * t3074;
    let t12325 = t3068 * t12324;
    let t12328 = -7.0 / 648.0 * t1098 * t12279 + 5.0 / 20736.0 * t9526 - t9530 / 4608.0 + t9535 + t9538 / 4608.0 - t9543 / 6912.0 + t9547 / 2304.0 + t12290 - t12294 + t1098 * t12295 / 108.0 + t1098 * t12298 / 216.0 + t1098 * t12301 / 36.0 - t1098 * t12304 / 72.0 - t1098 * t12307 / 144.0 - t1098 * t12310 / 48.0 + t4265 * t3107 / 864.0 + t4265 * t3103 / 432.0 - t12319 - t3067 * t12321 / 2304.0 - t3067 * t12325 / 4608.0;
    (t12328,)
}
