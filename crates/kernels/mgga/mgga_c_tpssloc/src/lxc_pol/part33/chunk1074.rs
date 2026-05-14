//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1074/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1074<F: Float>(t28372: F, t6605: F, t2628: F, t5585: F, t23096: F, t23106: F, t23108: F, t25065: F, t26619: F, t26621: F, t28357: F, t28360: F, t28362: F, t28364: F, t28366: F, t28368: F, t28370: F) -> (F, F) {
    let t28373 = t6605 * t28372;
    let t28375 = t2628 * t5585;
    let t28376 = t6605 * t28375;
    let t28378 = 0.40372756094140390854e-3 * t25065 - 0.20186378047070195427e-3 * t28357 + t28360 / 1536.0 - t28362 / 384.0 + t26619 - t26621 - t28364 / 1536.0 + t28366 / 768.0 - t28368 / 768.0 - t28370 / 1536.0 + t23096 - t23106 - 0.20186378047070195427e-3 * t28373 + 0.40372756094140390854e-3 * t28376 + t23108;
    (t28375, t28378)
}
