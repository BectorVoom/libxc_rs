//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 591/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk591<F: Float>(t1635: F, t665: F, t1364: F, t2024: F, t5898: F, t884: F, t2060: F, t5144: F, t1550: F, t5267: F, t903: F, t1627: F, t645: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8396 = t665 * t1635;
    let t8397 = t1364 * t8396;
    let t8399 = t2024 * t5898;
    let t8400 = t884 * t8399;
    let t8404 = t2060 * t5144;
    let t8405 = t1550 * t8404;
    let t8407 = t2060 * t5267;
    let t8408 = t903 * t8407;
    let t8410 = t645 * t1627;
    (t8396, t8397, t8399, t8400, t8404, t8405, t8407, t8408, t8410)
}
