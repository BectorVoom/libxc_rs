//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2159/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2159<F: Float>(t23204: F, t25216: F, t6562: F, t1519: F, t212: F, t23171: F, t6554: F, t23270: F, t25038: F, t258: F, t4119: F, t776: F) -> (F, F, F) {
    let t87910 = t6562 * t23204 * t25216;
    let t87911 = F::cast_from(0.82246703342411321824e-2_f64) * t87910;
    let t87915 = t23171 * t212 * t1519 * t6554;
    let t87920 = t25038 * t23270 * t258 * t4119 * t776;
    (t87911, t87915, t87920)
}
