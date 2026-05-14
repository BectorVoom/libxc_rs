//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1069/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1069<F: Float>(t107331: F, t107335: F, t107339: F, t107343: F, t107348: F, t107353: F, t107367: F, t107862: F, t544: F, t553: F, t90980: F, t90993: F, t91000: F, t97070: F, t97095: F, t97108: F, t97111: F, t97124: F, t97137: F, t97142: F) -> (F,) {
    let t107928 = 0.9869604401089358619e-1 * t97070 - 0.9869604401089358619e-1 * t107331 + 0.19739208802178717238e0 * t107335 + 0.9869604401089358619e-1 * t107339 + 0.9869604401089358619e-1 * t107343 + 0.46058153871750340221e0 * t97095 - 0.9869604401089358619e-1 * t107348 + 0.49348022005446793095e-1 * t90980 + 0.29608813203268075857e0 * t107353 + 0.23029076935875170111e0 * t97108 - 0.24674011002723396548e-1 * t97111 - 0.49348022005446793095e-1 * t90993 + t544 * t553 * t107862 - 0.46058153871750340221e0 * t97124 + 0.23029076935875170111e0 * t97137 + 0.9869604401089358619e-1 * t107367 + 0.24674011002723396548e-1 * t97142 - 0.38381794893125283518e0 * t91000;
    (t107928,)
}
