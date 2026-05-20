//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1100/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1100<F: Float>(t1297: F, t1390: F, t193: F, t2486: F, t3701: F, t3819: F, t3821: F, t3823: F, t3825: F, t3832: F, t3836: F, t3924: F, t533: F, t6324: F, t6329: F, t6330: F, t6347: F, t6399: F, t6400: F, t6463: F) -> F {
    let t6467 = t1390 * t193 * t533 * t6463 - t193 * t3701 * t533 * t6324 + F::new(3.0) * t1297 * t193 * t6347 + F::new(6.0) * t193 * t3924 * t6330 - t2486 + t3819 + t3821 + t3823 + t3825 - t3832 - t3836 + t6329 - t6399 - t6400;
    t6467
}
