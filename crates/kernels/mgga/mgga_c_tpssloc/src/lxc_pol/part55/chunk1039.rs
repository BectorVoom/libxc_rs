//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1039/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1039<F: Float>(t114172: F, t22892: F, t6891: F, t31220: F, t532: F, t22573: F, t8689: F, t32629: F, t580: F, t1404: F, t8919: F, t131: F, t32582: F, t2240: F, t9239: F, t9231: F) -> (F, F, F, F, F, F, F, F) {
    let t114316 = t22892 * t114172 * t6891;
    let t114418 = t532 * t31220;
    let t116135 = t8689 * t22573;
    let t117693 = t32629 * t580;
    let t117695 = t8919 * t1404;
    let t117709 = t32582 * t131;
    let t117710 = t2240 * t117709;
    let t117727 = t9239 * t117709;
    let t117734 = t9231 * t32582;
    (t114316, t114418, t116135, t117693, t117695, t117710, t117727, t117734)
}
