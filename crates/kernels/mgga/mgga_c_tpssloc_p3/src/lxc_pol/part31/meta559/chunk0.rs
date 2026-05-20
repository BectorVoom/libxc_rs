//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1787/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1787<F: Float>(t1519: F, t213: F, t225: F, t23168: F, t25229: F, t794: F, t23164: F, t6555: F, t7480: F, t81632: F, t23030: F, t25035: F) -> (F, F, F, F, F, F) {
    let t86873 = t213 * t1519 * t225;
    let t86886 = t23168 * t25229;
    let t86893 = t794 * t1519;
    let t86895 = t23164 * t86893 * t6555;
    let t86903 = t81632 * t7480;
    let t86911 = t23030 * t25035;
    (t86873, t86886, t86893, t86895, t86903, t86911)
}
