//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1315/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1315<F: Float>(t136: F, t1693: F, t799: F, t10672: F, t215: F, t1395: F, t2161: F, t226: F, t19766: F, t5567: F, t36098: F, t1379: F, t2407: F) -> (F, F, F, F, F, F) {
    let t63993 = t1693 * t799 * t136;
    let t63995 = t63993 * t215 * t10672;
    let t64007 = t1395 * t2161;
    let t64008 = t64007 * t226;
    let t64034 = t5567 * t19766;
    let t64039 = t36098 * t226;
    let t64042 = t1379 * t2407;
    (t63995, t64007, t64008, t64034, t64039, t64042)
}
