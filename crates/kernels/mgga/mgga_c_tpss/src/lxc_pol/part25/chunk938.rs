//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 938/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk938<F: Float>(t10982: F, t10989: F, t11049: F, t11002: F, t3857: F, t895: F, t1441: F, t2618: F, t2593: F, t1429: F, t2549: F, t2621: F, t3882: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11309 = F::new(0.34431666666666666666e0) * t10982;
    let t11312 = F::new(0.13892666666666666667e0) * t10989;
    let t11319 = F::new(0.27785333333333333334e0) * t11049;
    let t11328 = F::new(0.22954444444444444444e0) * t11002;
    let t11351 = t3857 * t895;
    let t11356 = t1441 * t2618;
    let t11362 = t1441 * t2593;
    let t11366 = t1429 * t2549;
    let t11399 = t3882 * t2621;
    (t11309, t11312, t11319, t11328, t11351, t11356, t11362, t11366, t11399)
}
