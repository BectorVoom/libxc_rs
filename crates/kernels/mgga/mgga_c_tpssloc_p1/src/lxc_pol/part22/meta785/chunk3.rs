//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2706/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2706<F: Float>(t12595: F, t12598: F, t12609: F, t12612: F, t16558: F, t17635: F, t19420: F, t19430: F, t20217: F, t20234: F, t2291: F, t2298: F, t39096: F, t39114: F, t3966: F, t4007: F, t4012: F, t5398: F, t607: F, t634: F, t638: F, t67060: F) -> F {
    let t75543 = F::new(3640.0) / F::new(81.0) * t39096 * t20234 * t607 - F::new(280.0) / F::new(9.0) * t19420 * t3966 - F::new(280.0) / F::new(9.0) * t12595 * t17635 + F::new(28.0) / F::new(3.0) * t12598 * t5398 + F::new(28.0) / F::new(3.0) * t4007 * t16558 + F::new(28.0) / F::new(9.0) * t2291 * t20217 * t607 - F::new(4.0) / F::new(3.0) * t634 * t67060 + F::new(3640.0) / F::new(81.0) * t39114 * t20234 * t607 + F::new(280.0) / F::new(9.0) * t19430 * t3966 + F::new(280.0) / F::new(9.0) * t12609 * t17635 + F::new(28.0) / F::new(3.0) * t12612 * t5398 + F::new(28.0) / F::new(3.0) * t4012 * t16558 + F::new(28.0) / F::new(9.0) * t2298 * t20217 * t607 + F::new(4.0) / F::new(3.0) * t638 * t67060;
    t75543
}
