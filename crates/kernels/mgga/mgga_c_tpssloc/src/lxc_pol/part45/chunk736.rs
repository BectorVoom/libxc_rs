//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 736/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk736<F: Float>(t22680: F, t22946: F, t533: F, t1390: F, t1983: F, t2379: F, t25: F, t1914: F, t193: F, t201: F, t2752: F) -> (F, F, F, F, F, F) {
    let t22947 = t22680 + t22946;
    let t22948 = t533 * t22947;
    let t22949 = t22948 * t1390;
    let t22950 = t1983 * t22949;
    let t22951 = t25 * t2379;
    let t22959 = t193 * t201 * t1914;
    let t22960 = t2752 * t25;
    (t22947, t22949, t22950, t22951, t22959, t22960)
}
