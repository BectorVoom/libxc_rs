//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 777/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk777<F: Float>(t35888: F, t3839: F, t35875: F, t851: F, t35924: F, t854: F, t35884: F, t3810: F, t35871: F, t305: F, t3899: F, t36172: F, t655: F) -> (F, F, F, F, F, F, F) {
    let t36184 = t3839 * t35888;
    let t36188 = t851 * t35875;
    let t36190 = t854 * t35924;
    let t36192 = t3810 * t35884;
    let t36194 = t851 * t35871;
    let t36200 = t305 * t3899;
    let t36201 = F::cast_from(0.22765842247987981715e0_f64) * t36200;
    let t36204 = t655 * t36172;
    (t36184, t36188, t36190, t36192, t36194, t36201, t36204)
}
