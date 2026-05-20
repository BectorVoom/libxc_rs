//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1439/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1439<F: Float>(t15376: F, t22069: F, t3447: F, t4908: F, t6123: F, t64811: F, t73274: F, t73276: F, t73279: F, t73287: F, t73290: F, t73307: F, t73314: F, t78043: F, t78047: F) -> F {
    let t78441 = -F::cast_from(0.1086419753086419753e-1_f64) * t73274 + F::cast_from(0.59259259259259259256e-2_f64) * t73276 - F::cast_from(0.11522633744855967078e-2_f64) * t73279 - F::cast_from(0.37037037037037037036e-3_f64) * t73287 - F::cast_from(0.33333333333333333332e-2_f64) * t73290 + F::cast_from(0.29629629629629629628e-2_f64) * t73307 + F::cast_from(0.29629629629629629628e-2_f64) * t73314 - F::cast_from(0.22222222222222222221e-2_f64) * t3447 * t4908 * t78047 - F::cast_from(0.99999999999999999996e-2_f64) * t3447 * t4908 * t78043 + F::cast_from(0.32592592592592592592e-1_f64) * t64811 * t6123 - F::cast_from(0.88888888888888888887e-2_f64) * t15376 * t22069;
    t78441
}
