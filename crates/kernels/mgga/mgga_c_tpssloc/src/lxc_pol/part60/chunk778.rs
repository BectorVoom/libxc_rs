//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 778/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk778<F: Float>(t6547: F, t8557: F, t2047: F, t234: F, t794: F, t8556: F, t6562: F, t814: F, t8543: F, t23204: F, t8547: F, t225: F, t8544: F, t8548: F, t2752: F, t8565: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31374 = t6547 * t8557;
    let t31375 = 0.19190897446562641759e-1 * t31374;
    let t31376 = t234 * t2047;
    let t31381 = t794 * t8556;
    let t31382 = t6562 * t31381;
    let t31383 = 0.41123351671205660912e-2 * t31382;
    let t31394 = t814 * t8543;
    let t31405 = t23204 * t8547;
    let t31406 = t6562 * t31405;
    let t31407 = 0.41123351671205660912e-2 * t31406;
    let t31423 = t8544 * t225;
    let t31425 = t6547 * t8548;
    let t31426 = 0.19190897446562641759e-1 * t31425;
    let t31434 = t8565 * t2752;
    (t31375, t31376, t31381, t31383, t31394, t31405, t31407, t31423, t31426, t31434)
}
