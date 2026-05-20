//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1298/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1298<F: Float>(t17620: F, t6717: F, t1933: F, t23479: F, t99660: F, t1015: F, t28581: F, t82895: F, t28577: F, t3128: F, t25641: F, t88451: F) -> (F, F, F, F, F) {
    let t99789 = t6717 * t17620;
    let t99796 = t1933 * t99660 * t23479;
    let t99799 = t82895 * t1015 * t28581;
    let t99802 = t82895 * t3128 * t28577;
    let t99813 = t88451 * t25641;
    (t99789, t99796, t99799, t99802, t99813)
}
