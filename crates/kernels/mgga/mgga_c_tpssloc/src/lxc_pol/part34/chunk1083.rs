//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1083/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1083<F: Float>(t105250: F, t105254: F, t108164: F, t108189: F, t108218: F, t108321: F, t1527: F, t17052: F, t17092: F, t2054: F, t26713: F, t2718: F, t29055: F, t5657: F, t5658: F, t67344: F, t7841: F, t7842: F, t84820: F, t855: F, t858: F, t86916: F) -> (F,) {
    let t108342 = -3.0 * t17052 * t7842 - t855 * t858 * (t108164 + t108189 + t108218 + t108321) - t67344 * t2054 + 0.9869604401089358619e-1 * t86916 + t84820 + 6.0 * t855 * t2718 * t29055 * t1527 + 6.0 * t855 * t2718 * t7841 * t5657 - 3.0 * t26713 * t5658 - 6.0 * t17092 * t7842 - 0.16449340668482264365e-1 * t105250 - 0.9869604401089358619e-1 * t105254;
    (t108342,)
}
