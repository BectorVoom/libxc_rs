//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1000/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1000<F: Float>(t11915: F, t22348: F, t1734: F, t1932: F, t475: F, t6260: F, t11883: F, t11889: F, t1751: F, t6224: F, t3612: F, t6218: F, t1755: F, t3625: F, t22327: F, t493: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22349 = t22348 * t11915;
    let t22354 = t1932 * t1734 * t475;
    let t22355 = t6260 * t22354;
    let t22358 = t22348 * t11883;
    let t22361 = t22348 * t11889;
    let t22364 = t1751 * t6224;
    let t22365 = t22364 * t3612;
    let t22368 = t3612 * t6218;
    let t22369 = t1755 * t22368;
    let t22372 = t22364 * t3625;
    let t22375 = t493 * t22327;
    (t22349, t22355, t22358, t22361, t22364, t22365, t22368, t22369, t22372, t22375)
}
