//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 666/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk666<F: Float>(t3897: F, t6388: F, t1825: F, t5348: F, t1380: F, t6415: F, t6420: F, t553: F, t6434: F, t1336: F, t1814: F, t1838: F, t1840: F, t5234: F, t544: F, t564: F, t6378: F) -> (F, F, F, F, F, F) {
    let t6448 = t3897 * t6388;
    let t6451 = t5348 * t1825;
    let t6454 = t1380 * t6415;
    let t6456 = t1380 * t6420;
    let t6458 = t553 * t6434;
    let t6460 = 2.0 * t1336 * t6448 - 2.0 * t1336 * t6451 - t1336 * t6454 - t1336 * t6456 + 2.0 * t1814 * t1840 - 2.0 * t1838 * t5234 + t544 * t6458 + t564 * t6378;
    (t6448, t6451, t6454, t6456, t6458, t6460)
}
