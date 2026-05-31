//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 299/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk299<F: Float>(t1368: F, t570: F, t552: F, t558: F, t1598: F, t521: F, t1714: F) -> (F, F, F, F, F) {
    let t1767 = t1368 * t570;
    let t1773 = t552 * t558;
    let t1776 = t1598 * t570;
    let t1794 = t521 * t521;
    let t1797 = F::cast_from(2.0_f64) * t1714;
    (t1767, t1773, t1776, t1794, t1797)
}
