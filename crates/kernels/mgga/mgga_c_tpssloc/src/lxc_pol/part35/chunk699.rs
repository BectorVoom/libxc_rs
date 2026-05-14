//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 699/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk699<F: Float>(t6914: F, t1878: F, t547: F, t1995: F, t2230: F, t213: F, t1999: F, t533: F, t556: F) -> (F, F, F, F, F) {
    let t6915 = 7.0 / 288.0 * t6914;
    let t6916 = t1878 * t547;
    let t6919 = t2230 * t1995;
    let t6920 = t6919 * t213;
    let t6921 = t6920 * t1999;
    let t6922 = 0.14130464632949136799e-2 * t6921;
    let t6924 = 1.0 / t556 / t533;
    (t6915, t6916, t6919, t6922, t6924)
}
