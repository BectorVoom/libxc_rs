//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1205/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1205<F: Float>(t1539: F, t6800: F, t100008: F, t100204: F, t100417: F, t1615: F, t21510: F, t21637: F, t21643: F, t23601: F, t23603: F, t23604: F, t23633: F, t23635: F, t25429: F, t25470: F, t25510: F, t25523: F, t25721: F, t28621: F, t28625: F, t28637: F, t5398: F, t6797: F, t7610: F, t82513: F, t82515: F, t82516: F, t82541: F, t82542: F, t89033: F) -> (F, F) {
    let t106028 = t6800 * t1539;
    let t106043 = 0.49348022005446793095e-1 * t82513 * t82515 * t21637 * t82516 - 0.49348022005446793095e-1 * t82513 * t82541 * t21637 * t82542 - 0.24674011002723396548e-1 * t23601 * t23603 * t21643 * t23604 + 0.10966227112321509577e-1 * t25429 * t25510 * t25721 * t21510 - 0.10966227112321509577e-1 * t25429 * t25470 * t28637 + 0.82246703342411321826e-2 * t23633 * t23635 * t5398 * t1615 * t6800 + 0.16449340668482264365e-1 * t23633 * t100204 * t106028 - 0.16449340668482264365e-1 * t89033 * t100008 - 0.49348022005446793095e-1 * t6797 * t25523 * t28625 - 0.24674011002723396548e-1 * t6797 * t100417 * t7610 - 0.24674011002723396548e-1 * t6797 * t25523 * t28621;
    (t106028, t106043)
}
