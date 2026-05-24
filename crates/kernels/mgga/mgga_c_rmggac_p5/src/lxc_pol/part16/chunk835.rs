//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 835/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk835<F: Float>(t3851: F, t39692: F, t3826: F, t3814: F, t40920: F, t2115: F, t41056: F, t2103: F, t41032: F, t36166: F, t2100: F, t41028: F) -> (F, F, F, F, F, F, F) {
    let t41338 = t3851 * t39692;
    let t41340 = t3826 * t39692;
    let t41342 = t3814 * t40920;
    let t41347 = t2115 * t41056;
    let t41355 = t2103 * t41032;
    let t41358 = F::cast_from(0.19513579069703984327e0_f64) * t36166;
    let t41363 = t2100 * t41028;
    (t41338, t41340, t41342, t41347, t41355, t41358, t41363)
}
