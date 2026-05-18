//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 990/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk990<F: Float>(t3787: F, t59: F, t240: F, t1336: F, t6943: F, t835: F, t6604: F, t6919: F, t6950: F, t6597: F, t6924: F, t281: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22759 = t3787 * t59;
    let t22760 = t22759 * t240;
    let t22761 = t1336 * t22760;
    let t22764 = t6943 * t835;
    let t22765 = t1336 * t22764;
    let t22779 = t6919 * t6604;
    let t22782 = t6950 * t835;
    let t22783 = t1336 * t22782;
    let t22791 = t6597 * t6924;
    let t22792 = t22791 * t281;
    (t22759, t22760, t22761, t22764, t22765, t22779, t22782, t22783, t22791, t22792)
}
