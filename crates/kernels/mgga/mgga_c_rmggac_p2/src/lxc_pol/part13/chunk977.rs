//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 977/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk977<F: Float>(t41150: F, t41404: F, t35959: F, t3851: F, t5260: F, t649: F, t35960: F, t5263: F, t27101: F, t39044: F, t39696: F, t5259: F) -> (F, F, F, F, F) {
    let t41405 = t41404 * t41150;
    let t41407 = t3851 * t35959;
    let t41409 = t41407 * t649 * t5260;
    let t41412 = t35960 * t649 * t5263;
    let t41436 = t27101 * t39044;
    let t41438 = t5259 * t39696;
    (t41405, t41409, t41412, t41436, t41438)
}
