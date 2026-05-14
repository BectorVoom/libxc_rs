//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1156/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1156<F: Float>(t115305: F, t6897: F, t80645: F, t8621: F, t22704: F, t31559: F, t81326: F, t2085: F, t212: F, t22642: F, t6890: F, t214: F, t7191: F, t22751: F, t31645: F, t31612: F, t6883: F) -> (F, F, F, F, F, F, F) {
    let t115306 = 0.63969658155208805863e-1 * t115305;
    let t115308 = t6897 * t80645 * t8621;
    let t115318 = t22704 * t81326 * t31559;
    let t115330 = t22642 * t212 * t2085 * t6890;
    let t115331 = 0.82246703342411321824e-2 * t115330;
    let t115332 = t214 * t7191;
    let t115339 = t22751 * t31645;
    let t115341 = t6883 * t31612;
    (t115306, t115308, t115318, t115331, t115332, t115339, t115341)
}
