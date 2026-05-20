//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1240/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1240<F: Float>(t15466: F, t15512: F, t15558: F, t15601: F, t15648: F, t15684: F, t15726: F, t15768: F, t493: F, t1215: F, t5052: F, t1246: F) -> (F, F, F) {
    let t15771 = t15466 + t15512 + t15558 + t15601 + t15648 + t15684 + t15726 + t15768;
    let t15772 = t493 * t15771;
    let t15776 = t5052 * t1215;
    let t15777 = t15776 * t1246;
    (t15771, t15772, t15777)
}
