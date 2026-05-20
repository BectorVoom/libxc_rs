//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2659/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2659<F: Float>(t12595: F, t12606: F, t12609: F, t12652: F, t16558: F, t19420: F, t19425: F, t19430: F, t19435: F, t2244: F, t2250: F, t2291: F, t2298: F, t39096: F, t39114: F, t4007: F, t4012: F, t5392: F, t5398: F, t55677: F, t55723: F, t607: F, t634: F, t638: F, t9321: F, t9330: F) -> F {
    let t55867 = F::new(3640.0) / F::new(81.0) * t39096 * t5392 * t2244 - F::new(1120.0) / F::new(27.0) * t12595 * t12652 - F::new(280.0) / F::new(27.0) * t19420 * t2250 + F::new(56.0) / F::new(9.0) * t2291 * t55723 + F::new(56.0) / F::new(9.0) * t4007 * t12606 - F::new(280.0) / F::new(27.0) * t9321 * t5398 * t2244 + F::new(56.0) / F::new(9.0) * t2291 * t16558 * t607 + F::new(28.0) / F::new(9.0) * t19425 * t2250 - F::new(4.0) / F::new(3.0) * t634 * t55677 + F::new(3640.0) / F::new(81.0) * t39114 * t5392 * t2244 + F::new(1120.0) / F::new(27.0) * t12609 * t12652 + F::new(280.0) / F::new(27.0) * t19430 * t2250 + F::new(56.0) / F::new(9.0) * t2298 * t55723 + F::new(56.0) / F::new(9.0) * t4012 * t12606 + F::new(280.0) / F::new(27.0) * t9330 * t5398 * t2244 + F::new(56.0) / F::new(9.0) * t2298 * t16558 * t607 + F::new(28.0) / F::new(9.0) * t19435 * t2250 + F::new(4.0) / F::new(3.0) * t638 * t55677;
    t55867
}
