//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 467/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk467<F: Float>(t2841: F, t321: F, t333: F, t352: F, t11599: F, t26: F, t551: F) -> (F, F, F, F, F, F, F) {
    let t11644 = t2841 * t321;
    let t11648 = t2841 * t333;
    let t11654 = t2841 * t352;
    let t11662 = t11599 * t321;
    let t11666 = t11599 * t333;
    let t11670 = t11599 * t352;
    let t11674 = t26 * t551;
    (t11644, t11648, t11654, t11662, t11666, t11670, t11674)
}
