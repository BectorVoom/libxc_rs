//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1293/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1293<F: Float>(t28565: F, t381: F, t23384: F, t28470: F, t28516: F, t25749: F, t7560: F, t225: F, t28594: F, t28519: F, t17667: F, t23537: F) -> (F, F, F, F, F, F, F) {
    let t99336 = t28565 * t381;
    let t99394 = t23384 * t28470;
    let t99398 = t23384 * t28516;
    let t99400 = t7560 * t25749;
    let t99415 = t28594 * t225;
    let t99439 = t23384 * t28519;
    let t99483 = t23537 * t17667;
    (t99336, t99394, t99398, t99400, t99415, t99439, t99483)
}
