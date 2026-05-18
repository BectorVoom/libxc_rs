//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1018/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1018<F: Float>(t1307: F, t1352: F, t2085: F, t22633: F, t6976: F, t1338: F, t31584: F, t3787: F, t8617: F, t114111: F, t114115: F, t114117: F, t114119: F, t114122: F, t114127: F, t1336: F, t31636: F, t31637: F, t3777: F, t3793: F, t3851: F) -> F {
    let t115484 = t22633 * t6976 * t2085 * t1307 * t1352;
    let t115486 = t1338 * t31584;
    let t115494 = t3787 * t8617;
    let t115498 = -t114111 + t114115 + F::new(0.3289868133696452873e-1) * t115484 - F::new(2.0) * t1336 * t115486 * t1352 - t1336 * t31636 * t3851 - F::new(2.0) * t3777 * t31637 - t114117 + t114119 + F::new(2.0) * t1336 * t115494 * t3793 + t114122 - t114127;
    t115498
}
