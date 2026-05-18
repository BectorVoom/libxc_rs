//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1281/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1281<F: Float>(t5464: F, t8180: F, t1453: F, t30293: F, t1449: F, t8184: F, t5488: F, t104: F, t1419: F, t656: F, t30297: F, t30063: F, t5480: F) -> (F, F, F, F, F, F, F) {
    let t30507 = t8180 * t5464;
    let t30510 = t30293 * t1453;
    let t30513 = t1453 * t1449;
    let t30514 = t8184 * t30513;
    let t30517 = t8180 * t5488;
    let t30521 = t656 * t1419 * t104;
    let t30524 = t30297 * t1449;
    let t30527 = t30063 * t5480;
    (t30507, t30510, t30514, t30517, t30521, t30524, t30527)
}
