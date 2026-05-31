//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2210/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2210<F: Float>(t10913: F, t12595: F, t12598: F, t12606: F, t12609: F, t12612: F, t1409: F, t2244: F, t2250: F, t2291: F, t2298: F, t39096: F, t39114: F, t3966: F, t4007: F, t4012: F, t45872: F, t607: F, t634: F, t638: F, t9258: F, t9288: F, t9321: F, t9330: F) -> F {
    let t45892 = F::cast_from(3640.0_f64) / F::cast_from(81.0_f64) * t39096 * t1409 * t9288 - F::cast_from(280.0_f64) / F::cast_from(9.0_f64) * t9321 * t3966 * t2244 - F::cast_from(280.0_f64) / F::cast_from(9.0_f64) * t12595 * t10913 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t2291 * t12606 * t607 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t12598 * t2250 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4007 * t9258 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t634 * t45872 + F::cast_from(3640.0_f64) / F::cast_from(81.0_f64) * t39114 * t1409 * t9288 + F::cast_from(280.0_f64) / F::cast_from(9.0_f64) * t9330 * t3966 * t2244 + F::cast_from(280.0_f64) / F::cast_from(9.0_f64) * t12609 * t10913 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t2298 * t12606 * t607 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t12612 * t2250 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4012 * t9258 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t638 * t45872;
    t45892
}
