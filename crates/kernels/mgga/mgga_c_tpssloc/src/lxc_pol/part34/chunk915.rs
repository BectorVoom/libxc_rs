//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 915/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk915<F: Float>(t1616: F, t17712: F, t4582: F, t1409: F, t5398: F, t4588: F, t10970: F, t21130: F, t248: F, t5681: F, t3071: F, t1539: F, t5873: F) -> (F, F, F, F, F, F) {
    let t21502 = t17712 * t1616;
    let t21503 = t4582 * t21502;
    let t21510 = t5398 * t1409;
    let t21511 = t4588 * t21510;
    let t21512 = t4582 * t21511;
    let t21516 = t248 * t10970 * t21130;
    let t21519 = t5681 * t1616;
    let t21520 = t3071 * t21519;
    let t21525 = t5873 * t1539;
    (t21503, t21510, t21512, t21516, t21520, t21525)
}
