//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1265/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1265<F: Float>(t1992: F, t20638: F, t22897: F, t20416: F, t6637: F, t6888: F, t6968: F, t22633: F, t26421: F, t6388: F, t1825: F, t26331: F, t6976: F, t97011: F, t6420: F, t107210: F, t1336: F, t20568: F, t28171: F, t5234: F, t544: F, t553: F, t6987: F, t81147: F, t81154: F, t90993: F, t91000: F, t97111: F, t97124: F, t97137: F, t97142: F, t97148: F, t97161: F) -> (F,) {
    let t107367 = t1992 * t22897 * t20638;
    let t107377 = t6888 * t6637 * t6968 * t20416;
    let t107381 = t22633 * t22897 * t26421 * t6388;
    let t107385 = t26331 * t6976 * t97011 * t1825;
    let t107389 = t22633 * t6976 * t26421 * t6420;
    let t107391 = -0.12337005501361698274e-1 * t97111 + 6.0 * t5234 * t28171 - 0.24674011002723396547e-1 * t90993 - t1336 * t6987 * t20568 - 0.23029076935875170111e0 * t97124 + 0.11514538467937585055e0 * t97137 + 0.49348022005446793095e-1 * t107367 + 0.12337005501361698274e-1 * t97142 - 0.19190897446562641759e0 * t91000 + 0.57572692339687925277e-1 * t97148 - 0.74022033008170189643e-1 * t97161 + t544 * t553 * t107210 - 0.16449340668482264365e-1 * t107377 - 0.9869604401089358619e-1 * t107381 - t81147 - 0.14804406601634037928e0 * t107385 + 0.49348022005446793095e-1 * t107389 + t81154;
    (t107391,)
}
