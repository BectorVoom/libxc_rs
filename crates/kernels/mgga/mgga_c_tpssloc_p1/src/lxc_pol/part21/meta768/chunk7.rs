//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2660/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2660<F: Float>(t12620: F, t12630: F, t12709: F, t1427: F, t1434: F, t19326: F, t19405: F, t19441: F, t2244: F, t2245: F, t2283: F, t2284: F, t2304: F, t33: F, t3998: F, t4018: F, t5392: F, t5393: F, t5427: F, t5442: F, t55723: F, t55751: F, t55801: F, t55867: F, t629: F, t642: F, t65: F, t66: F, t72: F, t80: F) -> F {
    let t55875 = t12709 * t1434 / F::new(12.0) + t3998 * t4018 / F::new(6.0) + t19405 * t642 / F::new(12.0) + t33 * (t55751 + t55801) * t80 / F::new(24.0) - t55723 * t65 * t80 / F::new(6.0) - t2244 * t5427 * t80 / F::new(12.0) - t5392 * t2283 * t80 / F::new(12.0) - t19326 * t642 / F::new(6.0) - t5393 * t2304 / F::new(12.0) + t1427 * t12620 / F::new(12.0) + t2284 * t5442 / F::new(24.0) + t629 * t19441 / F::new(12.0) + t66 * t72 * t55867 / F::new(24.0) - t12630 * t1434 / F::new(6.0) - t2245 * t5442 / F::new(12.0);
    t55875
}
