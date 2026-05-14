//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1062/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1062<F: Float>(t107056: F, t107214: F, t20029: F, t20044: F, t20613: F, t27009: F, t6461: F, t7194: F, t7925: F, t7937: F, t84423: F, t97529: F, t97537: F, t97548: F, t97571: F, t102948: F, t107250: F, t107260: F, t107265: F, t1375: F, t1842: F, t1843: F, t20662: F, t2092: F, t29299: F, t29360: F, t29372: F, t3887: F, t5215: F, t5321: F, t74860: F, t84659: F) -> (F, F) {
    let t107731 = 0.46058153871750340221e0 * t97529 - 0.3289868133696452873e-1 * t107056 + t84423 + 12.0 * t20029 * t7925 - 0.49348022005446793095e-1 * t107214 + 0.23029076935875170111e0 * t97537 - 0.23029076935875170111e0 * t97548 - 3.0 * t20044 * t7937 + 6.0 * t7194 * t20613 - 3.0 * t27009 * t6461 - 0.49348022005446793095e-1 * t97571;
    let t107772 = -3.0 * t102948 * t1843 - t84659 - 0.9869604401089358619e-1 * t107250 - 3.0 * t74860 * t2092 + 6.0 * t5215 * t29372 + 6.0 * t5321 * t29372 + 0.16449340668482264365e-1 * t107260 - t7194 * t20662 + 6.0 * t1375 * t3887 * t29360 * t1842 - 18.0 * t5321 * t29299 + 0.29608813203268075857e0 * t107265;
    (t107731, t107772)
}
