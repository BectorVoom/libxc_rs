//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1260/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1260<F: Float>(t1791: F, t19345: F, t5790: F, t6090: F, t1675: F, t5791: F, t6073: F, t19380: F, t1792: F, t18350: F, t18648: F, t18652: F, t18661: F, t18666: F, t18671: F, t18673: F, t18676: F, t19342: F, t19349: F, t19352: F, t5483: F, t5794: F, t6304: F) -> (F, F, F, F) {
    let t20264 = t1791 * t19345;
    let t20275 = t5790 * t6090;
    let t20276 = t1675 * t20275;
    let t20278 = t6073 * t5791;
    let t20282 = t1791 * t19380;
    let t20285 = F::new(40.0) / F::new(9.0) * t18671 + F::new(16.0) / F::new(9.0) * t18676 + F::new(10.0) * t18666 * t19342 + F::new(10.0) / F::new(3.0) * t18350 * t20264 + t18648 - F::new(8.0) / F::new(9.0) * t18652 - F::new(8.0) / F::new(9.0) * t18661 + F::new(10.0) / F::new(3.0) * t19349 * t18673 + t19352 * t1792 / F::new(3.0) + t6073 * t5794 / F::new(3.0) - F::new(8.0) / F::new(9.0) * t20276 - F::new(8.0) / F::new(9.0) * t20278 + t5483 * t6304 / F::new(3.0) + t1675 * t20282 / F::new(3.0);
    (t20264, t20275, t20282, t20285)
}
