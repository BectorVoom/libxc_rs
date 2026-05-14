//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1238/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1238<F: Float>(t1799: F, t22633: F, t22635: F, t97608: F, t1985: F, t20661: F, t6889: F, t6906: F, t1375: F, t20044: F, t2015: F, t2016: F, t20608: F, t28111: F, t40591: F, t5321: F, t74908: F, t7729: F, t80744: F, t90642: F, t90659: F, t90663: F, t97509: F) -> (F,) {
    let t107031 = t22633 * t22635 * t97608 * t1799;
    let t107044 = t1985 * t6889 * t6906 * t20661;
    let t107048 = -3.0 * t74908 * t2016 - t80744 + 0.24674011002723396548e-1 * t97509 - 0.9869604401089358619e-1 * t107031 + 24.0 * t1375 * t40591 * t2015 * t20608 + 6.0 * t5321 * t28111 + 0.24674011002723396547e-1 * t90642 - 0.19190897446562641759e0 * t90659 - 0.24674011002723396547e-1 * t90663 - 0.82246703342411321825e-2 * t107044 + 6.0 * t20044 * t7729;
    (t107048,)
}
