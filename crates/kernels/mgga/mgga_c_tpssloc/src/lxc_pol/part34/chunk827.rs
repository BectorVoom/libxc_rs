//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 827/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk827<F: Float>(t13909: F, t17784: F, t17809: F, t21430: F, t21433: F, t21447: F, t21453: F, t21459: F, t21463: F, t21469: F, t21473: F, t21476: F, t2986: F, t973: F, t21429: F, t225: F) -> (F, F) {
    let t21479 = 0.16666666666666666666e-2 * t2986 * t21430 - 0.83333333333333333331e-3 * t2986 * t21433 - 0.83333333333333333332e-3 * t973 * t21447 - 0.55555555555555555554e-3 * t17809 - 0.24999999999999999999e-2 * t973 * t21453 - 0.83333333333333333332e-3 * t973 * t21459 + 0.27777777777777777777e-3 * t973 * t21463 + 0.37037037037037037036e-3 * t17784 + 0.55555555555555555554e-3 * t13909 + 0.86419753086419753084e-3 * t973 * t21469 + 0.16666666666666666666e-2 * t973 * t21473 - 0.16666666666666666666e-2 * t2986 * t21476;
    let t21480 = t21429 + t21479;
    let t21481 = t21480 * t225;
    (t21480, t21481)
}
