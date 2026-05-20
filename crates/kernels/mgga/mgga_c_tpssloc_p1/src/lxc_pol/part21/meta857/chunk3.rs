//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3111/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3111<F: Float>(t64309: F, t64325: F, t64342: F, t64358: F, t64374: F, t64389: F, t64406: F, t64422: F, t1117: F, t51460: F, t51638: F, t3313: F, t3315: F, t63287: F) -> (F, F, F) {
    let t64425 = t64309 + t64325 + t64342 + t64358 + t64374 + t64389 + t64406 + t64422;
    let t64433 = F::cast_from(0.2069040516770936012e4_f64) * t51638 * t51460 * t1117;
    let t64436 = F::cast_from(0.32163958997385070134e2_f64) * t3313 * t63287 * t3315;
    (t64425, t64433, t64436)
}
