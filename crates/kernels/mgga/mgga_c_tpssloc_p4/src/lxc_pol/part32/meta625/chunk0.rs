//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2033/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2033<F: Float>(t86843: F, t4119: F, t857: F, t23168: F, t25342: F, t25345: F, t82038: F, t1519: F, t213: F, t225: F, t25229: F, t794: F) -> (F, F, F, F, F, F, F) {
    let t86844 = F::cast_from(0.38381794893125283518e-1_f64) * t86843;
    let t86849 = t857 * t4119;
    let t86868 = t23168 * t25342;
    let t86869 = F::cast_from(0.76763589786250567036e-1_f64) * t86868;
    let t86870 = t82038 * t25345;
    let t86873 = t213 * t1519 * t225;
    let t86886 = t23168 * t25229;
    let t86887 = F::cast_from(0.76763589786250567036e-1_f64) * t86886;
    let t86893 = t794 * t1519;
    (t86844, t86849, t86869, t86870, t86873, t86887, t86893)
}
