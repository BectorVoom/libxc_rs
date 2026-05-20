//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2122/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2122<F: Float>(t22986: F, t25054: F, t82159: F, t23168: F, t25229: F, t23222: F, t25224: F, t6552: F, t1519: F, t794: F, t23164: F, t6555: F) -> (F, F, F, F, F) {
    let t86884 = t22986 * t82159 * t25054;
    let t86886 = t23168 * t25229;
    let t86887 = F::cast_from(0.76763589786250567036e-1_f64) * t86886;
    let t86891 = t6552 * t25224 * t23222;
    let t86893 = t794 * t1519;
    let t86895 = t23164 * t86893 * t6555;
    (t86884, t86887, t86891, t86893, t86895)
}
