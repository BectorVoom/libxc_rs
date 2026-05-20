//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 822/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk822<F: Float>(t500: F, t1256: F, t193: F, t3258: F, t3261: F, t3268: F, t3310: F, t3318: F, t336: F, t3408: F, t3410: F, t3413: F, t3417: F, t3421: F, t3425: F, t3633: F, t3637: F) -> (F, F, F) {
    let t3639 = t500 * t500;
    let t3640 = F::new(1.0) / t3639;
    let t3643 = t1256 * t193 * t336 * t3633 - t193 * t336 * t3637 * t3640 - t3258 + t3261 - t3268 + t3310 + t3318 + t3408 + t3410 - t3413 + t3417 - t3421 - t3425;
    (t3639, t3640, t3643)
}
