//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1253/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1253<F: Float>(t2: F, t2436: F, t6148: F, t8096: F, t11454: F, t18098: F, t18083: F, t3969: F, t139: F, t19849: F, t3754: F, t928: F, t2697: F, t3749: F, t11457: F, t5620: F) -> (F, F, F, F, F, F, F) {
    let t64300 = t2436 * t2;
    let t64305 = t6148 * t8096;
    let t64325 = t18098 * t11454 / 1152.0;
    let t64343 = t18083 * t3969 / 324.0;
    let t64354 = t19849 * t139 * t928 * t3754 / 216.0;
    let t64358 = t19849 * t139 * t2697 * t3749 / 324.0;
    let t64401 = t5620 * t11457 / 864.0;
    (t64300, t64305, t64325, t64343, t64354, t64358, t64401)
}
