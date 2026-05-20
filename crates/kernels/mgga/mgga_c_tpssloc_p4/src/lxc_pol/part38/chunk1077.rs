//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1077/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1077<F: Float>(t2684: F, t4295: F, t13171: F, t860: F, t4265: F, t814: F, t829: F, t13377: F, t235: F, t2679: F, t4282: F, t4280: F, t808: F) -> (F, F, F, F, F, F) {
    let t13429 = t4295 * t2684;
    let t13431 = t860 * t13171;
    let t13433 = t814 * t4265;
    let t13434 = t13433 * t829;
    let t13448 = t235 * t13377;
    let t13450 = t4282 * t2679;
    let t13453 = t808 * t4280;
    (t13429, t13431, t13434, t13448, t13450, t13453)
}
