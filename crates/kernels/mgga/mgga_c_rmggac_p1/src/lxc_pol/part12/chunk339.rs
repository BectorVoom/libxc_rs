//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 339/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk339<F: Float>(t132: F, t270: F, t1322: F, t1325: F, t28: F, t1321: F) -> (F, F, F) {
    let t2040 = t132 * t270;
    let t2044 = t1322 * t1325;
    let t2045 = t2044 * t28;
    let t2046 = t1321 * t2045;
    (t2040, t2044, t2046)
}
