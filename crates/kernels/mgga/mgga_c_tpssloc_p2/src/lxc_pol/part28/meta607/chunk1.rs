//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1916/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1916<F: Float>(t1307: F, t6637: F, t6888: F, t90809: F, t1352: F, t22633: F, t6976: F, t90754: F, t5187: F, t562: F, t1799: F, t81129: F) -> (F, F, F, F, F) {
    let t90812 = t6888 * t6637 * t90809 * t1307;
    let t90816 = t22633 * t6976 * t90754 * t1352;
    let t90818 = t562 * t5187;
    let t90821 = t22633 * t6976 * t90818 * t1352;
    let t90825 = t6888 * t6637 * t81129 * t1799;
    (t90812, t90816, t90818, t90821, t90825)
}
