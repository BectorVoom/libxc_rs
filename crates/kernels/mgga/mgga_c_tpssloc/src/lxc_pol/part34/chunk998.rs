//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 998/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk998<F: Float>(t25064: F, t81902: F, t7521: F, t81632: F, t22690: F, t23171: F, t25319: F, t23143: F, t7525: F, t25316: F, t82038: F, t23228: F, t7488: F, t23030: F, t25205: F, t1519: F, t212: F, t6554: F) -> (F, F, F, F, F, F, F, F) {
    let t87445 = t81902 * t25064;
    let t87635 = t81632 * t7521;
    let t87653 = t23171 * t22690 * t25319;
    let t87666 = t23143 * t7525;
    let t87718 = t82038 * t25316;
    let t87779 = t23171 * t23228 * t7488;
    let t87898 = t23030 * t25205;
    let t87915 = t23171 * t212 * t1519 * t6554;
    (t87445, t87635, t87653, t87666, t87718, t87779, t87898, t87915)
}
