//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1217/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1217<F: Float>(t1675: F, t20275: F, t5791: F, t6073: F, t1791: F, t19380: F, t1792: F, t18350: F, t18648: F, t18652: F, t18661: F, t18666: F, t18671: F, t18673: F, t18676: F, t19342: F, t19349: F, t19352: F, t20264: F, t5483: F, t5794: F, t6304: F) -> (F, F, F, F) {
    let t20276 = t1675 * t20275;
    let t20278 = t6073 * t5791;
    let t20282 = t1791 * t19380;
    let t20285 = F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t18671 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t18676 + F::cast_from(10.0_f64) * t18666 * t19342 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t18350 * t20264 + t18648 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t18652 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t18661 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t19349 * t18673 + t19352 * t1792 / F::cast_from(3.0_f64) + t6073 * t5794 / F::cast_from(3.0_f64) - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t20276 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t20278 + t5483 * t6304 / F::cast_from(3.0_f64) + t1675 * t20282 / F::cast_from(3.0_f64);
    (t20276, t20278, t20282, t20285)
}
