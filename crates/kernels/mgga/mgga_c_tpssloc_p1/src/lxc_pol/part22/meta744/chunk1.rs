//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2469/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2469<F: Float>(t21390: F, t376: F, t10952: F, t1616: F, t17607: F, t17712: F, t21503: F, t21551: F, t3039: F, t3048: F, t3117: F, t42347: F, t4582: F, t4585: F, t4590: F, t4594: F, t4650: F, t61784: F, t61794: F, t61796: F, t62091: F) -> (F, F) {
    let t70273 = t376 * t21390;
    let t70296 = -t61784 / F::new(576.0) - t3117 * t21551 / F::new(768.0) + F::new(7.0) / F::new(1536.0) * t42347 * t4582 * t70273 * t4594 + t61794 / F::new(768.0) - t17607 * t4585 / F::new(768.0) + F::new(5.0) / F::new(4608.0) * t17607 * t4590 + F::new(5.0) / F::new(3456.0) * t61796 - t10952 * t21503 / F::new(1024.0) - t3039 * t4582 * t62091 * t1616 / F::new(1024.0) - t3039 * t4582 * t17712 * t4650 / F::new(1024.0) + t3048 * t21551 / F::new(144.0);
    (t70273, t70296)
}
