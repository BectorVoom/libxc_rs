//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1172/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1172<F: Float>(t22633: F, t31551: F, t2015: F, t7213: F, t3887: F, t2091: F, t3886: F) -> (F, F, F) {
    let t31552 = t22633 * t31551;
    let t31554 = t7213 * t2015;
    let t31555 = t3887 * t31554;
    let t31558 = t3886 * t2091;
    (t31552, t31555, t31558)
}
