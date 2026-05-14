//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 17/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk17<F: Float>(t40: F, rho1: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t41 = t40 / 2.0;
    let t42 = pow_1_3(t41);
    let t43 = t42 * t42;
    let t44 = t43 * t41;
    let t46 = rho1 * rho1;
    let t47 = pow_1_3(rho1);
    let t48 = t47 * t47;
    let t50 = 1.0 / t48 / t46;
    let t51 = sigma2 * t50;
    (t42, t43, t44, t46, t47, t48, t50, t51)
}
