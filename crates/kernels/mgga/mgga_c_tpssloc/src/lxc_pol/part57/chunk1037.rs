//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1037/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1037<F: Float>(t122537: F, t1799: F, t6637: F, t6888: F, t31618: F, t6347: F, t114064: F, t115433: F, t115494: F, t127361: F, t127362: F, t127371: F, t127375: F, t127381: F, t1336: F, t31636: F, t6388: F, t6415: F) -> F {
    let t128847 = t6888 * t6637 * t122537 * t1799;
    let t128851 = t6888 * t6637 * t31618 * t6347;
    let t128855 = t127361 + F::new(2.0) * t1336 * t115494 * t6388 + t127362 - t114064 - F::new(0.3289868133696452873e-1) * t128847 - F::new(0.16449340668482264365e-1) * t128851 - t1336 * t31636 * t6415 - t127371 - t127375 + t127381 + t115433;
    t128855
}
