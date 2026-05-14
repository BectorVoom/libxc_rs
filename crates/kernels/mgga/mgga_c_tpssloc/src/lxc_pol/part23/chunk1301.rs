//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1301/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1301<F: Float>(t11285: F, t1164: F, t44154: F, t78287: F, t22233: F, t4869: F, t21830: F, t11282: F, t3403: F, t18915: F, t6106: F, t6270: F, t1671: F, t71877: F, t18686: F, t6021: F) -> (F, F, F, F, F, F, F, F) {
    let t78310 = 0.12304822629859687989e5 * t1164 * t44154 * t78287 * t11285;
    let t78312 = 0.23392894490538584828e1 * t4869 * t22233;
    let t78314 = 0.20779030926817756511e3 * t4869 * t21830;
    let t78318 = 0.6233709278045326953e3 * t1164 * t11282 * t78287 * t3403;
    let t78320 = 0.10389515463408878255e3 * t18915 * t6106;
    let t78321 = t6270 * t6270;
    let t78327 = 4.0 * t71877 * t1671;
    let t78329 = 6.0 * t18686 * t6021;
    (t78310, t78312, t78314, t78318, t78320, t78321, t78327, t78329)
}
