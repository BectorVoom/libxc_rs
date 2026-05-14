//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 853/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk853<F: Float>(t2888: F, t5758: F, t225: F, t5849: F, t5851: F, t1040: F, t5904: F, t248: F, t3101: F, t5867: F, t1020: F, t135: F, t5889: F, t973: F, t5893: F, t5884: F) -> (F, F, F, F, F, F, F, F) {
    let t17547 = t5758 * t2888;
    let t17575 = t5849 * t225;
    let t17588 = t5851 * t225;
    let t17607 = t5904 * t1040;
    let t17611 = t248 * t3101 * t5867;
    let t17612 = t1020 * t17611;
    let t17615 = t135 * t5889;
    let t17616 = t973 * t17615;
    let t17620 = t135 * t5893;
    let t17621 = t973 * t17620;
    let t17624 = t135 * t5884;
    (t17547, t17575, t17588, t17607, t17612, t17616, t17621, t17624)
}
