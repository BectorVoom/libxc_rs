//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1195/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1195<F: Float>(t10662: F, t19696: F, t215: F, t10667: F, t19695: F, t19697: F, t5543: F, t136: F, t1693: F, t799: F, t10672: F, t1395: F, t2161: F, t226: F, t19766: F, t5567: F) -> (F, F, F, F, F, F, F) {
    let t63984 = t19696 * t215 * t10662;
    let t63987 = t19696 * t215 * t10667;
    let t63990 = t5543 * t19695 * t19697;
    let t63993 = t1693 * t799 * t136;
    let t63995 = t63993 * t215 * t10672;
    let t64007 = t1395 * t2161;
    let t64008 = t64007 * t226;
    let t64034 = t5567 * t19766;
    (t63984, t63987, t63990, t63995, t64007, t64008, t64034)
}
