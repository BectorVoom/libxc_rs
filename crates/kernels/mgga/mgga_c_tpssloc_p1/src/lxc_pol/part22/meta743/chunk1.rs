//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2465/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2465<F: Float>(t17607: F, t4571: F, t1011: F, t1019: F, t69923: F, t1025: F, t1622: F, t21405: F, t21580: F, t21609: F, t3048: F, t3117: F, t43211: F, t61659: F, t61663: F, t61665: F, t61710: F, t70132: F) -> F {
    let t70138 = t17607 * t4571;
    let t70148 = t69923 * t1011 * t1019;
    let t70151 = t3117 * t21609 / F::new(768.0) - t70132 / F::new(1152.0) + F::new(5.0) / F::new(432.0) * t3048 * t21580 - t61710 * t1622 / F::new(288.0) + t70138 / F::new(2304.0) - t3048 * t21609 / F::new(144.0) + t61659 / F::new(1152.0) - t61663 / F::new(2304.0) + t61665 / F::new(1536.0) - t43211 * t21405 / F::new(576.0) + t70148 * t1025 / F::new(3072.0);
    t70151
}
