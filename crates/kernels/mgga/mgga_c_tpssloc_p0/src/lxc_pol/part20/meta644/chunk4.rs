//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2362/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2362<F: Float>(t14234: F, t3070: F, t42488: F, t10390: F, t10408: F, t10413: F, t10445: F, t1046: F, t13527: F, t14218: F, t14219: F, t14228: F, t14230: F, t1611: F, t2244: F, t2250: F, t2770: F, t3071: F, t360: F, t369: F, t378: F, t42303: F, t48428: F, t48431: F, t48432: F, t48441: F, t48446: F, t48460: F, t68: F) -> F {
    let t48463 = t3070 * t42488 * t14234;
    let t48471 = t48431 + t48432 * t1046 / F::new(1536.0) + F::new(19.0) / F::new(1296.0) * t42303 + t48428 * t68 * t369 * t378 / F::new(3072.0) - t48441 / F::new(36.0) - F::new(209.0) / F::new(2592.0) * t1611 * t10445 * t378 + F::new(19.0) / F::new(864.0) * t48446 - t10413 * t3071 * t14218 * t14219 * t2250 / F::new(1536.0) - F::new(5.0) / F::new(4608.0) * t10413 * t10408 * t14218 * t360 * t2770 * t2244 - t48460 / F::new(576.0) + F::new(5.0) / F::new(3456.0) * t48463 - t10390 * t14230 / F::new(384.0) + F::new(5.0) / F::new(2304.0) * t3070 * t10408 * t13527 * t14228;
    t48471
}
