//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2211/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2211<F: Float>(t12677: F, t12681: F, t12684: F, t12687: F, t1414: F, t1420: F, t2262: F, t39: F, t39210: F, t3982: F, t3985: F, t43: F, t45872: F, t51: F, t55: F, t615: F, t9277: F, t9301: F, t9308: F) -> F {
    let t45931 = -F::cast_from(3080.0_f64) / F::cast_from(81.0_f64) * t9277 * t1414 + F::cast_from(220.0_f64) / F::cast_from(9.0_f64) * t2262 * t3985 - F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t615 * t12687 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t43 * t45872 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t1420 * t9301 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t1420 * t9308 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t55 * t45872 - t39210 + F::cast_from(220.0_f64) / F::cast_from(27.0_f64) * t2262 * t3982 - F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t615 * t12681 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t615 * t12684 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t615 * t12677;
    t45931
}
