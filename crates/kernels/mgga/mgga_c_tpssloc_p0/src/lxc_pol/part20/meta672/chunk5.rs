//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2531/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2531<F: Float>(t50853: F, t43768: F, t43770: F, t44249: F, t50846: F, t50848: F, t50851: F, t50859: F, t50863: F, t50867: F, t50871: F, t50875: F) -> F {
    let t51271 = F::cast_from(0.34731666666666666667e0_f64) * t50853;
    let t51279 = -F::cast_from(0.30872592592592592592e0_f64) * t50846 - F::cast_from(0.20839e0_f64) * t50848 + F::cast_from(0.104195e0_f64) * t50851 + t51271 + F::cast_from(0.69463333333333333332e-1_f64) * t43768 - F::cast_from(0.41678000000000000001e0_f64) * t43770 + t44249 - F::cast_from(0.34731666666666666667e-1_f64) * t50859 - F::cast_from(0.125034e1_f64) * t50863 + F::cast_from(0.62517e0_f64) * t50867 + F::cast_from(0.187551e1_f64) * t50871 + F::cast_from(0.20839e0_f64) * t50875;
    t51279
}
