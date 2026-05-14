//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1160/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1160<F: Float>(t5464: F, t8180: F, t1453: F, t30293: F, t1449: F, t8184: F, t5488: F, t104: F, t1419: F, t656: F, t30297: F, t30063: F, t5480: F, t5484: F, t29903: F, t30048: F, t30279: F, t30291: F, t30301: F, t64: F, t8128: F, t8137: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30507 = t8180 * t5464;
    let t30510 = t30293 * t1453;
    let t30513 = t1453 * t1449;
    let t30514 = t8184 * t30513;
    let t30517 = t8180 * t5488;
    let t30521 = t656 * t1419 * t104;
    let t30524 = t30297 * t1449;
    let t30527 = t30063 * t5480;
    let t30530 = t8184 * t5484;
    let t30533 = -t30048 - 4.0 / 3.0 * t30279 - 10.0 / 9.0 * t30291 + 10.0 / 9.0 * t30301 - 3.0 / 4.0 * t29903 * t30507 - 5.0 / 6.0 * t8128 * t30510 + 5.0 / 6.0 * t8128 * t30514 + t8128 * t30517 / 4.0 - 5.0 / 9.0 * t64 * t30521 + 25.0 / 36.0 * t8137 * t30524 - 5.0 / 36.0 * t8137 * t30527 - 5.0 / 24.0 * t8137 * t30530;
    (t30507, t30510, t30514, t30517, t30521, t30524, t30527, t30530, t30533)
}
