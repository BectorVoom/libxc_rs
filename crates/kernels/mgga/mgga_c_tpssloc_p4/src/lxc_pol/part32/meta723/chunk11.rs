//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2317/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2317<F: Float>(t18375: F, t7339: F, t27599: F, t4997: F, t18360: F, t18364: F, t18397: F, t18401: F, t19002: F, t19016: F, t24741: F, t27617: F, t4950: F, t4980: F, t4984: F, t5014: F, t5030: F, t86324: F, t86327: F, t95566: F, t95623: F, t95627: F) -> F {
    let t104048 = t7339 * t18375;
    let t104050 = t27599 * t4997;
    let t104056 = t95566 * t4950 / F::new(216.0) - t24741 * t18360 / F::new(1152.0) + F::new(5.0) / F::new(6912.0) * t24741 * t18364 - t27599 * t5014 / F::new(144.0) - t27617 * t5030 / F::new(1152.0) + F::new(5.0) / F::new(3456.0) * t24741 * t19016 - t86324 * t19002 / F::new(576.0) + t86327 * t18397 / F::new(1152.0) - t24741 * t18401 / F::new(576.0) + t104048 / F::new(2304.0) - t104050 / F::new(216.0) - t95623 * t4980 / F::new(72.0) + t95627 * t4984 / F::new(144.0);
    t104056
}
