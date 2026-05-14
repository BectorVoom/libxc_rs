//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 961/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk961<F: Float>(t135: F, t5889: F, t973: F, t5893: F, t5884: F, t4593: F, t4650: F, t4582: F, t5398: F, t607: F) -> (F, F, F, F, F) {
    let t17615 = t135 * t5889;
    let t17616 = t973 * t17615;
    let t17620 = t135 * t5893;
    let t17621 = t973 * t17620;
    let t17624 = t135 * t5884;
    let t17625 = t973 * t17624;
    let t17631 = t4593 * t4650;
    let t17632 = t4582 * t17631;
    let t17635 = t5398 * t607;
    (t17616, t17621, t17625, t17632, t17635)
}
