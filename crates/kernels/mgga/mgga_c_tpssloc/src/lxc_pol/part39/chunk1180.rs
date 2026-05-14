//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1180/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1180<F: Float>(t110075: F, t30281: F, t29895: F, t30285: F, t30304: F, t29900: F, t30308: F, t110082: F, t110290: F, t110292: F, t110294: F, t110314: F, t110601: F, t110602: F, t1449: F, t1453: F, t2: F, t2332: F, t2350: F, t2354: F, t2358: F, t29903: F, t30056: F, t30063: F, t30175: F, t30297: F, t4067: F, t662: F, t8128: F, t8137: F, t8180: F, t8184: F, t86592: F, t86595: F, t86598: F) -> (F,) {
    let t111056 = 4.0 * t110075 * t30281;
    let t111058 = 20.0 / 9.0 * t29895 * t30285;
    let t111077 = 20.0 / 9.0 * t29895 * t30304;
    let t111079 = 20.0 / 27.0 * t29900 * t30308;
    let t111096 = 2.0 * t110290 + 10.0 / 27.0 * t110294 + 3.0 * t110082 * t8180 * t86592 + 5.0 / 18.0 * t8128 * t30063 * t1453 * t2350 - 5.0 / 4.0 * t29903 * t8184 * t1449 * t2332 + 5.0 / 108.0 * t8137 * t110314 * t1449 * t2350 + t111056 - t111058 - 3.0 / 2.0 * t29903 * t8180 * t86595 - 3.0 / 4.0 * t29903 * t8180 * t86598 + 5.0 / 6.0 * t8128 * t8184 * t4067 * t662 + 5.0 / 12.0 * t8128 * t8184 * t1453 * t2354 - 25.0 / 18.0 * t8128 * t30297 * t30056 - t111077 + t111079 + 5.0 / 12.0 * t8128 * t8184 * t1449 * t2358 - 5.0 / 6.0 * t110601 * t8184 * t110602 - 5.0 / 36.0 * t8137 * t30063 * t1449 * t2354 + 5.0 / 18.0 * t30175 * t30063 * t2 * t662 - 20.0 / 9.0 * t110292;
    (t111096,)
}
