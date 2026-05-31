//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2685/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2685<F: Float>(t1352: F, t16224: F, t16306: F, t20448: F, t20563: F, t3803: F, t54556: F, t54582: F, t54612: F, t57308: F, t57310: F, t57324: F, t57383: F, t57392: F, t57396: F, t57398: F, t57407: F, t57409: F) -> F {
    let t74806 = -F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t3803 * t16224 * t20563 * t1352 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t57308 - F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t57310 - t54556 - F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t57324 + F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t57383 + F::cast_from(455.0_f64) / F::cast_from(216.0_f64) * t54582 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t57392 + F::cast_from(35.0_f64) / F::cast_from(64.0_f64) * t57396 - F::cast_from(35.0_f64) / F::cast_from(192.0_f64) * t57398 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t57407 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t57409 - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t3803 * t16224 * t16306 * t20448 + t54612;
    t74806
}
