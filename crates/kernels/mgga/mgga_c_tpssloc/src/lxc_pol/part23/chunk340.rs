//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 340/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk340<F: Float>(t3: F, t576: F, t112: F, t582: F, t586: F, t589: F, t593: F, t596: F, t600: F, t4: F, t581: F) -> (F, F, F, F) {
    let t1398 = t3 * t576;
    let t1401 = t576 * t112;
    let t1406 = -t582 - t586 - t589 - t593 - t596 - t600;
    let t1408 = -t4 - t581;
    (t1398, t1401, t1406, t1408)
}
