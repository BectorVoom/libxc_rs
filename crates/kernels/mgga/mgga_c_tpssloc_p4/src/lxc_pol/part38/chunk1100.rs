//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1100/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1100<F: Float>(t13615: F, t901: F, t2815: F, t4370: F, t896: F, t2807: F, t4378: F, t2798: F, t4362: F, t10595: F, t1547: F, t2799: F) -> (F, F, F, F, F, F) {
    let t13616 = t901 * t13615;
    let t13623 = t2815 * t4370;
    let t13624 = t13623 * t896;
    let t13626 = t4378 * t2807;
    let t13629 = t2798 * t4370;
    let t13630 = t13629 * t896;
    let t13632 = t4362 * t2807;
    let t13634 = t10595 * t1547;
    let t13635 = t13634 * t2799;
    (t13616, t13624, t13626, t13630, t13632, t13635)
}
