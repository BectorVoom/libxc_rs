//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1046/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1046<F: Float>(t109: F, t12451: F, t3701: F, t24486: F, t576: F, t111: F, t7222: F, t81437: F, t81440: F, t81443: F, t81445: F, t81447: F, t81450: F, t81452: F, t112: F, t24447: F, t24007: F) -> (F, F, F, F, F, F) {
    let t110 = 1.0 < t109;
    let t83911 = t3701 * t12451;
    let t84031 = t576 * t24486;
    let t84033 = t7222 * t111;
    let t84036 = 308.0 / 27.0 * t81437;
    let t84044 = piecewise3(t110, 0.0, -t84036 - 22.0 / 3.0 * t81440 - 4.0 * t81443 + 2.0 * t81445 - 3.0 / 2.0 * t81447 + 3.0 / 2.0 * t81450 - t81452 / 4.0);
    let t84078 = t24447 * t112;
    let t84097 = t24007 * t111;
    (t83911, t84031, t84033, t84044, t84078, t84097)
}
