//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1135/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1135<F: Float>(t12012: F, t12156: F, t12477: F, t1307: F, t1388: F, t1390: F, t193: F, t3719: F, t3918: F, t39529: F, t39531: F, t39533: F, t39539: F, t39541: F, t39549: F, t39563: F, t39570: F, t39572: F, t39577: F, t39582: F, t39585: F, t5126: F, t571: F) -> F {
    let t39586 = F::new(24.0) * t12012 * t1307 * t5126 * t571 + F::new(24.0) * t12156 * t1388 * t1390 * t193 - F::new(18.0) * t12477 * t3719 * t3918 + F::new(12.0) * t1307 * t3918 * t39577 - t39529 - t39531 - t39533 + t39539 - t39541 + t39549 + t39563 + t39570 - t39572 + t39582 - t39585;
    t39586
}
