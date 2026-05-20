//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1837/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1837<F: Float>(t23270: F, t2379: F, t25038: F, t25053: F, t22986: F, t25054: F, t82159: F, t23168: F, t25229: F, t23222: F, t25224: F, t6552: F) -> (F, F, F, F) {
    let t86881 = t25038 * t23270 * t25053 * t2379;
    let t86884 = t22986 * t82159 * t25054;
    let t86886 = t23168 * t25229;
    let t86891 = t6552 * t25224 * t23222;
    (t86881, t86884, t86886, t86891)
}
