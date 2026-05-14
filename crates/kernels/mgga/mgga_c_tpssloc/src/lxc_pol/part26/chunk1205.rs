//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1205/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1205<F: Float>(t11605: F, t225: F, t1184: F, t3470: F, t3597: F, t3599: F, t2122: F, t7303: F, t85660: F, t3590: F, t7299: F, t24571: F, t24574: F, t11607: F, t1186: F, t11925: F, t1238: F, t2154: F, t24633: F, t24638: F, t24877: F, t24883: F, t27799: F, t3471: F, t3477: F, t3593: F, t45350: F, t7283: F, t7300: F, t7302: F, t7392: F) -> (F, F, F) {
    let t85674 = t225 * t11605;
    let t85683 = t3470 * t1184;
    let t85687 = t3597 * t3599;
    let t85688 = t2122 * t85687;
    let t85701 = t85660 * t7303;
    let t85707 = t7299 * t3590;
    let t85711 = t24574 * t24571;
    let t85713 = -0.49348022005446793095e-1 * t7283 * t7300 * t85674 * t11607 + 24.0 * t1238 * t45350 * t2154 * t11607 - 0.24674011002723396548e-1 * t7283 * t85683 * t27799 - 0.49348022005446793095e-1 * t7283 * t1186 * t85688 + 0.24674011002723396548e-1 * t7283 * t3471 * t24638 + 0.24674011002723396548e-1 * t7283 * t3477 * t24638 - 0.82246703342411321826e-2 * t7283 * t24633 * t24883 + 0.54831135561607547884e-2 * t85701 + 6.0 * t3593 * t24877 - 3.0 * t11925 * t7392 - 0.24674011002723396548e-1 * t7283 * t85707 * t7302 - 0.82246703342411321826e-2 * t85711;
    (t85683, t85687, t85713)
}
