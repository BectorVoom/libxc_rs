//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1021/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1021<F: Float>(t3787: F, t59: F, t240: F, t1336: F, t6943: F, t835: F, t6604: F, t6919: F, t6950: F, t6597: F, t6924: F, t281: F, t547: F, t6546: F, t2230: F, t213: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
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
    let t22797 = t6546 * t547;
    let t22803 = t2230 * t6924;
    let t22804 = t22803 * t213;
    (t22759, t22760, t22761, t22764, t22765, t22779, t22782, t22783, t22791, t22792, t22797, t22803, t22804)
}
