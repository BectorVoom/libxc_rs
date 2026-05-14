//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 856/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk856<F: Float>(t120341: F, t32721: F, t19956: F, t31170: F, t5248: F, t550: F, t19871: F, t6420: F, t6936: F, t6943: F, t22759: F, t6388: F, t1985: F, t214: F, t225: F, t28107: F, t567: F) -> (F, F, F, F, F, F) {
    let t127285 = t120341 * t32721;
    let t127289 = t31170 * t5248 * t19956 * t550;
    let t127293 = t31170 * t5248 * t19871 * t550;
    let t127296 = t6936 * t6943 * t6420;
    let t127299 = t6936 * t22759 * t6388;
    let t127316 = 0.16449340668482264365e-1 * t1985 * t214 * t28107 * t225 * t567;
    (t127285, t127289, t127293, t127296, t127299, t127316)
}
