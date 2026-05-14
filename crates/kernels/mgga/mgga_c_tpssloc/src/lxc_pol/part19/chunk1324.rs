//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1324/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1324<F: Float>(t3507: F, t491: F, t11721: F, t23508: F, t1009: F, t11598: F, t1243: F, t3590: F, t11714: F, t476: F, t42341: F, t44696: F, t3508: F, t11883: F, t3493: F, t11889: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t44699 = t3507 * t3507;
    let t44700 = t491 * t44699;
    let t44701 = t23508 * t11721;
    let t44706 = t11598 * t1009;
    let t44707 = t44706 * t1243;
    let t44710 = t3590 * t3507;
    let t44722 = 1.0 / t11714 / t476;
    let t44724 = t44696 * t42341 * t44722;
    let t44725 = t3508 * t3508;
    let t44726 = t23508 * t44725;
    let t44730 = t11883 * t3493;
    let t44741 = t11889 * t3493;
    (t44699, t44700, t44701, t44706, t44707, t44710, t44722, t44724, t44725, t44726, t44730, t44741)
}
