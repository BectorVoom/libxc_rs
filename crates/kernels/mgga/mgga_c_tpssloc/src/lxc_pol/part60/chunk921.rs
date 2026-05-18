//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 921/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk921<F: Float>(t2085: F, t6387: F, t225: F, t29290: F, t29293: F, t29287: F, t111: F, t29485: F, t112: F, t29865: F, t23030: F, t30660: F) -> (F, F, F, F, F, F, F) {
    let t102801 = t2085 * t6387;
    let t102917 = t29290 * t225;
    let t102922 = t29293 * t225;
    let t102948 = t29287 * t225;
    let t104990 = t29485 * t111;
    let t105105 = t29865 * t112;
    let t112676 = F::new(0.52089578783527170489e-1) * t23030 * t30660;
    (t102801, t102917, t102922, t102948, t104990, t105105, t112676)
}
