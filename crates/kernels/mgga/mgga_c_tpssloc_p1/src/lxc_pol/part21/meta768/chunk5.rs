//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2658/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2658<F: Float>(t12606: F, t12695: F, t12699: F, t12702: F, t1420: F, t16558: F, t19368: F, t19369: F, t19372: F, t19377: F, t19390: F, t19397: F, t2244: F, t2250: F, t2267: F, t2274: F, t39: F, t39159: F, t39168: F, t3990: F, t51: F, t5392: F, t5398: F, t607: F, t615: F, t9287: F, t9300: F) -> F {
    let t55801 = F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t39 * t2267 * t16558 * t607 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t39 * t19377 * t2250 - F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t39 * t9287 * t5398 * t2244 - F::cast_from(80.0_f64) / F::cast_from(27.0_f64) * t1420 * t12699 - F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t1420 * t12702 - F::cast_from(20.0_f64) / F::cast_from(81.0_f64) * t1420 * t12695 + F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t51 * t19390 * t2250 + F::cast_from(5.0_f64) / F::cast_from(162.0_f64) * t51 * t39168 * t5392 * t2244 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t51 * t3990 * t12606 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t51 * t2274 * t16558 * t607 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t51 * t19397 * t2250 + F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t51 * t9300 * t5398 * t2244 - F::cast_from(80.0_f64) / F::cast_from(27.0_f64) * t615 * t19372 + F::cast_from(20.0_f64) / F::cast_from(81.0_f64) * t615 * t19369 - F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t39 * t19368 * t2250 + F::cast_from(5.0_f64) / F::cast_from(162.0_f64) * t39 * t39159 * t5392 * t2244;
    t55801
}
