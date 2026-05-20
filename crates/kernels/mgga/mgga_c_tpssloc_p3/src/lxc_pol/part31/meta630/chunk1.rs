//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1890/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1890<F: Float>(t19930: F, t6952: F, t1831: F, t91191: F, t26257: F, t5314: F, t28100: F, t80853: F, t80855: F, t22788: F, t6431: F, t6427: F) -> (F, F, F, F, F, F) {
    let t97340 = t6952 * t19930;
    let t97342 = t91191 * t1831;
    let t97344 = t26257 * t5314;
    let t97347 = t80853 * t80855 * t28100;
    let t97352 = t22788 * t6431;
    let t97354 = t22788 * t6427;
    (t97340, t97342, t97344, t97347, t97352, t97354)
}
