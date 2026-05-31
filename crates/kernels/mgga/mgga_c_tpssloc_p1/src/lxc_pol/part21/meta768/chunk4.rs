//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2657/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2657<F: Float>(t12606: F, t12705: F, t1420: F, t19378: F, t19381: F, t2262: F, t2267: F, t2274: F, t2275: F, t2278: F, t39: F, t39210: F, t3981: F, t43: F, t45970: F, t45974: F, t51: F, t5408: F, t5411: F, t5416: F, t55: F, t55677: F, t55716: F, t55723: F, t615: F) -> F {
    let t55751 = -F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t45970 * t55716 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t45974 * t55716 - t39210 + F::cast_from(220.0_f64) / F::cast_from(81.0_f64) * t2262 * t5408 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t39 * t2267 * t55723 + F::cast_from(220.0_f64) / F::cast_from(27.0_f64) * t2262 * t5411 - F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t615 * t19381 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t43 * t55677 - F::cast_from(220.0_f64) / F::cast_from(27.0_f64) * t5416 * t2278 + F::cast_from(220.0_f64) / F::cast_from(81.0_f64) * t5416 * t2275 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t1420 * t12705 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t55 * t55677 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t51 * t2274 * t55723 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t39 * t3981 * t12606 - F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t615 * t19378;
    t55751
}
