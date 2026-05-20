//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2164/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2164<F: Float>(t22705: F, t22852: F, t236: F, t550: F, t6414: F, t80784: F, t80792: F, t80794: F, t80826: F, t80837: F, t80848: F, t91282: F, t91284: F, t91287: F, t91290: F, t91301: F, t97352: F, t97354: F, t97359: F, t97361: F, t97363: F, t97367: F) -> F {
    let t97372 = t22852 * t22705 * t236 * t6414 * t550;
    let t97376 = -t97352 / F::new(384.0) + F::new(5.0) / F::new(384.0) * t97354 + F::cast_from(0.16821981705891829522e-4_f64) * t80784 - F::cast_from(0.52708876011794399171e-3_f64) * t80792 + F::new(119.0) / F::new(6912.0) * t80794 - t80826 + F::new(5.0) / F::new(384.0) * t97359 + F::new(5.0) / F::new(192.0) * t97361 - F::new(7.0) / F::new(2304.0) * t97363 - F::cast_from(0.6728792682356731809e-4_f64) * t97367 + F::cast_from(0.33643963411783659045e-4_f64) * t97372 + F::cast_from(0.10093189023535097713e-3_f64) * t80837 - t80848 + t91282 + t91284 + t91287 - F::cast_from(0.16956557559538964159e-1_f64) * t91290 - t91301;
    t97376
}
