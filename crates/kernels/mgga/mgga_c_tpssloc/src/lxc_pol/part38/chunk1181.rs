//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1181/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1181<F: Float>(t64: F, t91: F, t9365: F, t1453: F, t95: F, t2331: F, t29900: F, t30168: F, t656: F, t9576: F, t30176: F, t29895: F, t30159: F, t110111: F, t110141: F, t110144: F, t110146: F, t110158: F, t1444: F, t2: F, t29907: F, t29911: F, t29922: F, t30175: F, t4049: F, t4067: F, t8128: F, t8137: F) -> (F,) {
    let t110520 = t64 * t9365 * t91;
    let t110521 = t95 * t1453;
    let t110526 = t64 * t2331 * t91;
    let t110531 = 50.0 / 27.0 * t29900 * t30168;
    let t110532 = t9576 * t656;
    let t110533 = t110532 * t30176;
    let t110542 = 4.0 / 3.0 * t29895 * t30159;
    let t110549 = -20.0 / 9.0 * t110111 - 5.0 / 2.0 * t110520 * t110521 * t29911 + 5.0 / 9.0 * t110526 * t4049 * t29911 - t110531 + 125.0 / 72.0 * t110533 - 25.0 / 27.0 * t8137 * t110158 * t1444 + 25.0 / 36.0 * t30175 * t29922 * t2 - t110542 - 5.0 / 6.0 * t8128 * t29907 * t4067 + 44.0 / 9.0 * t110141 - 110.0 / 27.0 * t110144 - 2.0 / 3.0 * t110146;
    (t110549,)
}
