//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1170/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1170<F: Float>(t104: F, t1419: F, t110334: F, t110336: F, t111056: F, t111058: F, t111077: F, t111079: F, t19525: F, t19529: F, t2: F, t30175: F, t30293: F, t30297: F, t4067: F, t666: F, t8128: F, t8137: F, t8180: F, t8184: F) -> (F,) {
    let t111711 = t1419 * t104;
    let t111715 = t111056 - t111058 - t111077 + t111079 + 22.0 / 9.0 * t110334 - 55.0 / 27.0 * t110336 - 25.0 / 36.0 * t30175 * t30297 * t2 - 5.0 / 24.0 * t8137 * t8184 * t19525 - 5.0 / 6.0 * t8128 * t30293 * t4067 + t8128 * t8180 * t19529 / 4.0 + 10.0 / 9.0 * t8128 * t111711 * t666;
    (t111715,)
}
