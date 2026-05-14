//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1131/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1131<F: Float>(t43603: F, t68: F, t11008: F, t225: F, t3215: F, t112: F, t12512: F, t111: F, t3931: F, t2311: F, t671: F, t2363: F, t649: F, t89: F, t9416: F, t88: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43604 = t68 * t43603;
    let t43619 = t11008 * t225;
    let t43636 = t3215 * t3215;
    let t43637 = 1.0 / t43636;
    let t45557 = t12512 * t112;
    let t45560 = t3931 * t111;
    let t45602 = t2311 * t671;
    let t45637 = t649 * t2363;
    let t45640 = t89 * t9416;
    let t45814 = t88 * t9416;
    (t43604, t43619, t43637, t45557, t45560, t45602, t45637, t45640, t45814)
}
