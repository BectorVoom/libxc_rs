//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 898/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk898<F: Float>(t22923: F, t22925: F, t532: F, t7216: F, t193: F, t201: F, t2056: F) -> (F, F, F, F) {
    let t24156 = 0.12793931631041761173e0 * t22923;
    let t24157 = 0.52089578783527170489e-1 * t22925;
    let t24175 = t532 * t7216;
    let t24191 = t193 * t201 * t2056;
    (t24156, t24157, t24175, t24191)
}
