//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 729/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk729<F: Float>(t1322: F, t235: F, t36632: F, t20: F, t1311: F, t1325: F, t3054: F, t641: F, t70383: F, t13809: F, t7345: F, t13815: F, t2169: F, t7553: F) -> (F, F, F, F) {
    let t70585 = t235 * t36632 * t1322;
    let t70604 = t20 * t20;
    let t70610 = t1311 * t70604 * t3054 * t1322 * t1325 * t70383 * t641;
    let t70612 = t7345 * t13809;
    let t70618 = t7553 * t13815 * t2169;
    (t70585, t70610, t70612, t70618)
}
