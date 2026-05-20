//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2304/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2304<F: Float>(t5187: F, t562: F, t1352: F, t22633: F, t6976: F, t1799: F, t6637: F, t6888: F, t81129: F, t22881: F, t16049: F, t1992: F, t81027: F) -> (F, F, F, F, F) {
    let t90818 = t562 * t5187;
    let t90821 = t22633 * t6976 * t90818 * t1352;
    let t90825 = t6888 * t6637 * t81129 * t1799;
    let t90829 = t6888 * t6637 * t22881 * t5187;
    let t90832 = t1992 * t81027 * t16049;
    (t90818, t90821, t90825, t90829, t90832)
}
