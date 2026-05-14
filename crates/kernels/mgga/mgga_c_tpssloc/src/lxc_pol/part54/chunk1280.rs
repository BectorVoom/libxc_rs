//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1280/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1280<F: Float>(t1458: F, t31518: F, t652: F, t1873: F, t92090: F, t120908: F, t2039: F, t33211: F, t7056: F, t122660: F, t26135: F, t88: F, t33596: F, t31537: F, t7801: F, t31717: F) -> (F, F, F, F, F, F, F, F, F) {
    let t122713 = 2.0 * t652 * t31518 * t1458;
    let t122718 = t92090 * t1873;
    let t122719 = t120908 * t2039;
    let t122720 = t33211 * t7056;
    let t122721 = t122660 * t2039;
    let t122722 = t88 * t26135;
    let t122723 = t122722 * t2039;
    let t122724 = t33596 * t7056;
    let t122725 = t31537 * t7801;
    let t122726 = t31717 * t7801;
    (t122713, t122718, t122719, t122720, t122721, t122723, t122724, t122725, t122726)
}
