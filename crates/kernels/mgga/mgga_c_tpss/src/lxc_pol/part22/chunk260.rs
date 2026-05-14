//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 260/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk260<F: Float>(t198: F, t207: F, t654: F, t679: F, t684: F, t693: F, t726: F, t729: F, t734: F, t739: F, t740: F, t750: F, t821: F, t823: F) -> (F,) {
    let t826 = t198 * t207 * t821 * t823 + 3.0 * t198 * t740 * t750 + t654 + t679 + t684 + t693 + t726 + t729 - t734 - t739;
    (t826,)
}
