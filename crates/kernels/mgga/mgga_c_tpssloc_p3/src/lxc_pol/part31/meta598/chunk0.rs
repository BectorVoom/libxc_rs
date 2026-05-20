//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1843/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1843<F: Float>(t26739: F, t2752: F, t193: F, t201: F, t7844: F, t86843: F, t86868: F, t225: F, t26722: F, t86886: F, t86895: F, t2053: F, t40889: F) -> (F, F, F, F, F, F, F, F) {
    let t92276 = t26739 * t2752;
    let t92319 = t193 * t201 * t7844;
    let t92375 = F::cast_from(0.76763589786250567036e-1_f64) * t86843;
    let t92382 = F::cast_from(0.15352717957250113407e0_f64) * t86868;
    let t92386 = t26722 * t225;
    let t92390 = F::cast_from(0.15352717957250113407e0_f64) * t86886;
    let t92393 = F::cast_from(0.3289868133696452873e-1_f64) * t86895;
    let t92394 = t40889 * t2053;
    (t92276, t92319, t92375, t92382, t92386, t92390, t92393, t92394)
}
