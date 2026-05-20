//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1315/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1315<F: Float>(t10390: F, t10394: F, t10398: F, t1041: F, t10428: F, t10433: F, t10884: F, t10891: F, t10904: F, t10915: F, t10919: F, t10932: F, t14187: F, t2960: F, t3048: F, t3071: F, t3073: F, t42460: F, t42468: F, t42478: F, t42481: F, t42483: F, t42490: F, t42496: F, t4582: F, t884: F) -> F {
    let t42499 = F::new(2.0) / F::new(27.0) * t42460 + F::new(8.0) / F::new(27.0) * t2960 * t10932 - t10904 * t10428 / F::new(24.0) + t10891 * t10433 / F::new(48.0) + F::new(5.0) / F::new(864.0) * t1041 * t4582 * t14187 * t42468 + t3048 * t10915 / F::new(36.0) - F::new(5.0) / F::new(216.0) * t3048 * t10919 - t42478 / F::new(576.0) + t42481 / F::new(576.0) + t42483 * t3071 * t10884 * t884 / F::new(1152.0) + F::new(5.0) / F::new(1728.0) * t42490 + t10390 * t10394 / F::new(384.0) + t10390 * t10398 / F::new(384.0) - t42496 * t3073 / F::new(36.0);
    t42499
}
