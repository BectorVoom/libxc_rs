//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2112/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2112<F: Float>(t1862: F, t2240: F, t5392: F, t1409: F, t605: F, t3966: F, t72: F, t79: F, t2235: F, t5399: F, t17635: F, t19334: F) -> (F, F, F, F, F, F) {
    let t96547 = t2240 * t5392 * t1862;
    let t96551 = t605 * t1409 * t1862;
    let t96553 = t72 * t79 * t3966;
    let t96556 = t2235 * t5399;
    let t96559 = t605 * t17635;
    let t96562 = t605 * t19334;
    (t96547, t96551, t96553, t96556, t96559, t96562)
}
