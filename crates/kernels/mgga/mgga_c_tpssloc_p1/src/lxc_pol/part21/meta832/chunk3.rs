//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2935/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2935<F: Float>(t48155: F, t48157: F, t48159: F, t48161: F, t48163: F, t48165: F, t48167: F, t60163: F, t60166: F, t60168: F, t60171: F, t60173: F, t60189: F) -> F {
    let t61138 = -F::new(2.0) / F::new(9.0) * t60163 + t60166 / F::new(6.0) - F::new(10.0) / F::new(27.0) * t60168 - F::new(2.0) / F::new(3.0) * t60171 + F::new(5.0) / F::new(27.0) * t60173 - F::new(40.0) / F::new(27.0) * t48155 + F::new(20.0) / F::new(81.0) * t48157 + F::new(8.0) / F::new(9.0) * t48159 + F::new(4.0) / F::new(9.0) * t48161 + F::new(4.0) / F::new(9.0) * t48163 - F::new(4.0) / F::new(27.0) * t48165 - F::new(2.0) / F::new(27.0) * t48167 - F::new(4.0) * t60189;
    t61138
}
