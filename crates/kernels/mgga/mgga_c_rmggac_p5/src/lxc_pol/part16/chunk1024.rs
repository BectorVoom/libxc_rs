//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1024/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1024<F: Float>(t1707: F, t2064: F, t3928: F, t1550: F, t6522: F, t7778: F, t1990: F, t9826: F, t6355: F, t9005: F, t11905: F, t2301: F) -> (F, F, F, F, F) {
    let t47390 = t3928 * t2064 * t1707;
    let t47393 = t1550 * t7778 * t6522;
    let t47405 = t9826 * t1990;
    let t47408 = t6355 * t9005;
    let t47410 = t11905 * t2301;
    (t47390, t47393, t47405, t47408, t47410)
}
