//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2696/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2696<F: Float>(t1390: F, t16497: F, t193: F, t19577: F, t19631: F, t20063: F, t20067: F, t20077: F, t20085: F, t3918: F, t39483: F, t5122: F, t5126: F, t5160: F, t5161: F, t5308: F, t533: F, t54409: F, t6330: F, t74086: F, t74470: F, t74868: F, t74899: F, t74929: F, t75183: F) -> F {
    let t75198 = F::new(18.0) * t5126 * t20067 * t5308 + F::new(9.0) * t3918 * t5122 * t19631 + t54409 + t74086 + F::new(18.0) * t5126 * t16497 * t6330 + t39483 + t193 * t533 * (t74868 + t74899 + t74929 + t75183) * t1390 - F::new(3.0) * t5160 * t5161 * t20063 - F::new(18.0) * t5126 * t20077 * t5308 + F::new(18.0) * t3918 * t20085 * t19577 - t74470;
    t75198
}
