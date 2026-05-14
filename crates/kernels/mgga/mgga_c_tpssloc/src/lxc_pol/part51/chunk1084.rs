//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1084/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1084<F: Float>(t1393: F, t1869: F, t2075: F, t2096: F, t2314: F, t31246: F, t31753: F, t31761: F, t31769: F, t31771: F, t31774: F, t31778: F, t6515: F, t6539: F, t7042: F, t7156: F, t7218: F, t7220: F, t8450: F, t8529: F, t8604: F) -> (F,) {
    let t31779 = t1393 * t8604 - t1869 * t7156 - t2075 * t6515 + t2096 * t31246 - 2.0 * t2314 * t8529 - 2.0 * t6539 * t7042 + t7218 * t8450 - t7220 * t8450 - t31753 + t31761 - t31769 - t31771 - t31774 + t31778;
    (t31779,)
}
