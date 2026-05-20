//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1174/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1174<F: Float>(t1369: F, t40292: F, t12361: F, t3866: F, t12336: F, t12379: F, t12392: F, t12397: F, t12404: F, t12429: F, t1341: F, t1343: F, t1363: F, t1367: F, t3778: F, t3858: F, t3876: F, t39892: F, t40206: F, t40271: F, t40282: F, t40285: F, t40287: F, t820: F) -> F {
    let t40293 = t40292 * t1369;
    let t40295 = t3866 * t12361;
    let t40303 = F::new(7.0) / F::new(1152.0) * t40206 - t3778 * t12392 / F::new(768.0) - t1341 * t1343 * t820 * t40271 / F::new(3072.0) - t3778 * t12379 / F::new(768.0) - t12397 * t3858 / F::new(512.0) + F::new(119.0) / F::new(288.0) * t40282 + F::new(7.0) / F::new(96.0) * t40285 - t40287 * t1369 / F::new(192.0) - t12336 * t3876 / F::new(128.0) - F::new(119.0) / F::new(288.0) * t40293 + F::new(7.0) / F::new(288.0) * t40295 - t1363 * t1367 * t820 * t39892 / F::new(768.0) + t12429 * t12404 / F::new(64.0);
    t40303
}
