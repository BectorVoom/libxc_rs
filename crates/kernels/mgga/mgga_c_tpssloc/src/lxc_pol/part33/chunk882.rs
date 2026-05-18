//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 882/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk882<F: Float>(t1020: F, t17611: F, t135: F, t5889: F, t973: F, t5893: F, t5884: F, t248: F, t3101: F, t5878: F, t3039: F, t3051: F, t5685: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17612 = t1020 * t17611;
    let t17615 = t135 * t5889;
    let t17616 = t973 * t17615;
    let t17620 = t135 * t5893;
    let t17621 = t973 * t17620;
    let t17624 = t135 * t5884;
    let t17625 = t973 * t17624;
    let t17655 = t248 * t3101 * t5878;
    let t17656 = t3039 * t17655;
    let t17659 = t248 * t3051 * t5685;
    (t17612, t17615, t17616, t17620, t17621, t17624, t17625, t17655, t17656, t17659)
}
