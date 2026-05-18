//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1419/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1419<F: Float>(t22633: F, t26421: F, t6420: F, t6976: F, t107210: F, t107367: F, t107377: F, t107381: F, t107385: F, t1336: F, t20568: F, t28171: F, t5234: F, t544: F, t553: F, t6987: F, t81147: F, t81154: F, t90993: F, t91000: F, t97111: F, t97124: F, t97137: F, t97142: F, t97148: F, t97161: F) -> F {
    let t107389 = t22633 * t6976 * t26421 * t6420;
    let t107391 = -F::new(0.12337005501361698274e-1) * t97111 + F::new(6.0) * t5234 * t28171 - F::new(0.24674011002723396547e-1) * t90993 - t1336 * t6987 * t20568 - F::new(0.23029076935875170111e0) * t97124 + F::new(0.11514538467937585055e0) * t97137 + F::new(0.49348022005446793095e-1) * t107367 + F::new(0.12337005501361698274e-1) * t97142 - F::new(0.19190897446562641759e0) * t91000 + F::new(0.57572692339687925277e-1) * t97148 - F::new(0.74022033008170189643e-1) * t97161 + t544 * t553 * t107210 - F::new(0.16449340668482264365e-1) * t107377 - F::new(0.9869604401089358619e-1) * t107381 - t81147 - F::new(0.14804406601634037928e0) * t107385 + F::new(0.49348022005446793095e-1) * t107389 + t81154;
    t107391
}
