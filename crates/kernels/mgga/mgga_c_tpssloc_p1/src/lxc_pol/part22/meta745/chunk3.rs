//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2476/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2476<F: Float>(t10408: F, t1616: F, t17187: F, t17980: F, t3070: F, t3071: F, t42552: F, t4575: F, t4650: F, t49691: F, t49693: F, t50193: F, t5677: F, t61950: F, t61981: F, t62013: F, t62032: F, t62038: F) -> F {
    let t70432 = t61981 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3888.0_f64) * t42552 + t50193 * t17980 / F::cast_from(1024.0_f64) + t3070 * t3071 * t17187 * t1616 / F::cast_from(1536.0_f64) + t61950 * t4575 / F::cast_from(1536.0_f64) + t62013 / F::cast_from(1152.0_f64) - t49691 - t49693 + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t3070 * t10408 * t5677 * t4650 + t62032 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t62038;
    t70432
}
