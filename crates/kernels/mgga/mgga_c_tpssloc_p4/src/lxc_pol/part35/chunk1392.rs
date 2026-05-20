//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1392/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1392<F: Float>(t20284: F, t71: F, t33: F, t75284: F, t1437: F, t5441: F, t72: F, t3953: F, t5392: F, t1433: F, t5389: F, t5399: F) -> (F, F, F, F, F, F) {
    let t106800 = t71 * t20284;
    let t106804 = t75284 * t33;
    let t106813 = t72 * t5441 * t1437;
    let t106816 = t3953 * t5392;
    let t106826 = t72 * t1433 * t5389;
    let t106829 = t3953 * t5399;
    (t106800, t106804, t106813, t106816, t106826, t106829)
}
