//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 805/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk805<F: Float>(t5612: F, t7101: F, t24218: F, t24220: F, t24221: F, t25065: F, t25077: F, t25080: F, t28357: F, t28360: F, t28362: F, t28364: F, t28366: F, t28368: F, t28370: F, t28373: F, t28376: F) -> (F, F) {
    let t29010 = t7101 * t5612;
    let t29025 = F::new(0.80745512188280781706e-3) * t25065 - F::new(0.40372756094140390853e-3) * t28357 + t28360 / F::new(768.0) - t28362 / F::new(192.0) + F::new(7.0) / F::new(144.0) * t25077 - F::new(7.0) / F::new(576.0) * t25080 - t28364 / F::new(768.0) + t28366 / F::new(384.0) - t28368 / F::new(384.0) - t28370 / F::new(768.0) + t24218 - t24220 - F::new(0.40372756094140390853e-3) * t28373 + F::new(0.80745512188280781706e-3) * t28376 + t24221;
    (t29010, t29025)
}
