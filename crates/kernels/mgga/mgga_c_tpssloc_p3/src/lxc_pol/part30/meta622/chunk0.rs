//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2021/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2021<F: Float>(t86868: F, t25345: F, t82038: F, t1519: F, t213: F, t225: F, t23168: F, t25229: F, t794: F, t23164: F, t6555: F, t7480: F, t81632: F) -> (F, F, F, F, F, F, F) {
    let t86869 = F::cast_from(0.76763589786250567036e-1_f64) * t86868;
    let t86870 = t82038 * t25345;
    let t86873 = t213 * t1519 * t225;
    let t86886 = t23168 * t25229;
    let t86887 = F::cast_from(0.76763589786250567036e-1_f64) * t86886;
    let t86893 = t794 * t1519;
    let t86895 = t23164 * t86893 * t6555;
    let t86896 = F::cast_from(0.16449340668482264365e-1_f64) * t86895;
    let t86903 = t81632 * t7480;
    (t86869, t86870, t86873, t86887, t86893, t86896, t86903)
}
