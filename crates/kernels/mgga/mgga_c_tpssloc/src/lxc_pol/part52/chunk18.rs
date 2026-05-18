//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 18/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk18<F: Float>(t41: F, t43: F, rho1: F) -> (F, F, F, F, F) {
    let t44 = t43 * t41;
    let t46 = rho1 * rho1;
    let t47 = pow_1_3::<f64>(rho1);
    let t48 = t47 * t47;
    let t50 = F::new(1.0) / t48 / t46;
    (t44, t46, t47, t48, t50)
}
