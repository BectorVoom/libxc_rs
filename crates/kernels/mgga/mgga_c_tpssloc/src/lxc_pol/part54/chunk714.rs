//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 714/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk714<F: Float>(t477: F, t491: F, t1090: F, t7362: F, t1186: F, t2148: F, t50: F, t6794: F, t131: F, t467: F, t1009: F, t461: F, t1209: F) -> (F, F, F, F, F, F, F, F) {
    let t7363 = t477 * t491;
    let t7364 = t7363 * t1090;
    let t7365 = t7362 * t7364;
    let t7368 = t1186 * t2148;
    let t7371 = t50 * t6794;
    let t7372 = t7371 * t131;
    let t7373 = t7372 * t467;
    let t7374 = t461 * t1009;
    let t7375 = t7374 * t1209;
    (t7363, t7364, t7365, t7368, t7371, t7372, t7373, t7375)
}
