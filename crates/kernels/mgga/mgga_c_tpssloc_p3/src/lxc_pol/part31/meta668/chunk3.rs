//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1968/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1968<F: Float>(t81789: F, t87237: F, t87243: F, t87268: F, t92590: F, t92599: F, t92603: F, t92607: F, t92614: F, t92615: F, t98690: F, t98694: F, t98696: F, t98701: F, t98703: F, t98707: F, t98709: F, t98711: F) -> F {
    let t101425 = -F::new(7.0) / F::new(1152.0) * t98690 - t92590 - t87237 - F::new(119.0) / F::new(1728.0) * t87243 + t92599 + t92603 + t92607 - F::cast_from(0.63250651214153279004e-2_f64) * t81789 + F::new(7.0) / F::new(72.0) * t98694 + F::cast_from(0.16956557559538964158e-1_f64) * t98696 - t87268 - t92614 + t92615 + F::cast_from(0.80745512188280781706e-3_f64) * t98701 - t98703 / F::new(24.0) - F::cast_from(0.24223653656484234512e-2_f64) * t98707 - F::new(7.0) / F::new(24.0) * t98709 - F::cast_from(0.11869590291677274911e0_f64) * t98711;
    t101425
}
