//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2627/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2627<F: Float>(t11665: F, t11668: F, t11678: F, t11731: F, t11770: F, t14735: F, t15438: F, t15708: F, t15750: F, t3577: F, t4723: F, t52911: F, t53366: F, t53453: F, t53456: F, t53468: F, t53470: F, t53472: F, t53476: F) -> F {
    let t53478 = F::new(5.0) / F::new(2304.0) * t11678 * t11668 * t4723 * t53366 - t53453 - t15438 * t11770 / F::new(1024.0) - t53456 / F::new(81.0) + F::new(5.0) / F::new(2304.0) * t11665 * t15750 + F::new(5.0) / F::new(2304.0) * t3577 * t11668 * t14735 * t15708 + F::new(5.0) / F::new(4608.0) * t3577 * t11668 * t4723 * t52911 - t53468 / F::new(2304.0) - t53470 / F::new(1152.0) - t53472 * t11731 / F::new(512.0) - t53476 / F::new(576.0);
    t53478
}
