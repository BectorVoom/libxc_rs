//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1305/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1305<F: Float>(t15376: F, t22069: F, t3447: F, t4908: F, t6123: F, t64811: F, t73274: F, t73276: F, t73279: F, t73287: F, t73290: F, t73307: F, t73314: F, t78043: F, t78047: F, t4900: F, t4904: F, t64821: F, t73169: F, t73330: F, t73386: F, t73389: F, t73395: F, t73417: F, t73420: F, t73424: F, t78031: F, t78039: F) -> (F, F) {
    let t78441 = -0.1086419753086419753e-1 * t73274 + 0.59259259259259259256e-2 * t73276 - 0.11522633744855967078e-2 * t73279 - 0.37037037037037037036e-3 * t73287 - 0.33333333333333333332e-2 * t73290 + 0.29629629629629629628e-2 * t73307 + 0.29629629629629629628e-2 * t73314 - 0.22222222222222222221e-2 * t3447 * t4908 * t78047 - 0.99999999999999999996e-2 * t3447 * t4908 * t78043 + 0.32592592592592592592e-1 * t64811 * t6123 - 0.88888888888888888887e-2 * t15376 * t22069;
    let t78460 = 0.11111111111111111111e-2 * t3447 * t73169 * t4904 - 0.22222222222222222221e-2 * t73330 + 0.88888888888888888887e-2 * t73386 - 0.11111111111111111111e-2 * t73389 + 0.11111111111111111111e-2 * t73395 - 0.14814814814814814815e-2 * t73417 + 0.11111111111111111111e-2 * t73420 - 0.74074074074074074072e-3 * t64821 + 0.88888888888888888887e-2 * t73424 + 0.14814814814814814815e-2 * t3447 * t4900 * t78031 + 0.13333333333333333333e-1 * t3447 * t4900 * t78039;
    (t78441, t78460)
}
