//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1293/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1293<F: Float>(t30395: F, t576: F, t2212: F, t5363: F, t1395: F, t8299: F, t1453: F, t2: F, t104: F, t1419: F, t110334: F, t110336: F, t111056: F, t111058: F, t111077: F, t111079: F, t19525: F, t19529: F, t30175: F, t30293: F, t30297: F, t4067: F, t666: F, t8128: F, t8137: F, t8180: F, t8184: F) -> (F, F, F, F, F) {
    let t111308 = F::new(2.0) * t576 * t30395;
    let t111310 = F::new(2.0) * t5363 * t2212;
    let t111312 = F::new(2.0) * t1395 * t8299;
    let t111331 = t1453 * t2;
    let t111711 = t1419 * t104;
    let t111715 = t111056 - t111058 - t111077 + t111079 + F::new(22.0) / F::new(9.0) * t110334 - F::new(55.0) / F::new(27.0) * t110336 - F::new(25.0) / F::new(36.0) * t30175 * t30297 * t2 - F::new(5.0) / F::new(24.0) * t8137 * t8184 * t19525 - F::new(5.0) / F::new(6.0) * t8128 * t30293 * t4067 + t8128 * t8180 * t19529 / F::new(4.0) + F::new(10.0) / F::new(9.0) * t8128 * t111711 * t666;
    (t111308, t111310, t111312, t111331, t111715)
}
