//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1076/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1076<F: Float>(t101694: F, t105574: F, t105578: F, t105582: F, t105586: F, t105596: F, t105601: F, t1510: F, t16673: F, t20861: F, t24255: F, t26661: F, t5612: F, t7837: F, t812: F, t84851: F, t87140: F, t87155: F, t98399: F, t98416: F, t98420: F, t98446: F, t98488: F) -> (F,) {
    let t108189 = -3.0 * t812 * t26661 * t5612 - 3.0 * t812 * t101694 * t1510 - 0.9869604401089358619e-1 * t105574 - 0.9869604401089358619e-1 * t105578 + 0.29608813203268075857e0 * t105582 + 0.24674011002723396548e-1 * t98399 + 0.9869604401089358619e-1 * t105586 + 6.0 * t812 * t24255 * t20861 - t84851 + 0.46058153871750340221e0 * t98416 + 0.9869604401089358619e-1 * t87140 - 0.46058153871750340221e0 * t98420 + 0.9869604401089358619e-1 * t105596 - 0.9869604401089358619e-1 * t98446 - 0.49348022005446793095e-1 * t105601 - 3.0 * t16673 * t7837 + 0.15626873635058151147e0 * t87155 + 0.11514538467937585055e0 * t98488;
    (t108189,)
}
