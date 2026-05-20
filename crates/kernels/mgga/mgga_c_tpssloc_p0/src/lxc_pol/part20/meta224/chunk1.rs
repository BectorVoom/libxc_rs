//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1300/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1300<F: Float>(t2262: F, t2268: F, t2271: F, t39: F, t44: F, t51: F, t615: F, t618: F, t9277: F, t9289: F, t9293: F, t9296: F, t9301: F, t9305: F, t9308: F, t9311: F) -> F {
    let t9312 = -F::new(1232.0) / F::new(27.0) * t9277 * t44 + F::new(220.0) / F::new(9.0) * t2262 * t618 - F::new(20.0) / F::new(9.0) * t615 * t2268 - F::new(20.0) / F::new(3.0) * t615 * t2271 - F::new(5.0) / F::new(108.0) * t39 * t9289 + F::new(5.0) / F::new(6.0) * t39 * t9293 + F::new(5.0) / F::new(6.0) * t39 * t9296 + F::new(5.0) / F::new(108.0) * t51 * t9301 + F::new(5.0) / F::new(6.0) * t51 * t9305 - F::new(5.0) / F::new(6.0) * t51 * t9308 + t9311;
    t9312
}
