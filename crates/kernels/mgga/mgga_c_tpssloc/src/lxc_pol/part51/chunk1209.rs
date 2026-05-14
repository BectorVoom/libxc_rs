//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1209/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1209<F: Float>(t31759: F, t7685: F, t31300: F, t91655: F, t120954: F, t120958: F, t120962: F, t120964: F, t120966: F, t120968: F, t120973: F, t1976: F, t27145: F, t27170: F, t31246: F, t33133: F, t652: F, t7156: F, t7220: F, t7451: F, t7904: F, t8450: F) -> (F,) {
    let t120975 = 3.0 * t7685 * t31759;
    let t120979 = 3.0 * t91655 * t31300;
    let t120980 = -2.0 * t1976 * t27170 * t652 + t27145 * t8450 + 3.0 * t31246 * t7904 - t33133 * t7220 - t7156 * t7451 - t120954 + t120958 - t120962 - t120964 - t120966 - t120968 - t120973 + t120975 - t120979;
    (t120980,)
}
