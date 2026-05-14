//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 835/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk835<F: Float>(t135: F, t457: F, t11529: F, t1709: F, t1174: F, t11588: F, t1714: F, t1716: F, t698: F, t1420: F, t1887: F, t337: F) -> (F, F, F, F, F, F, F) {
    let t15281 = t135 * t457;
    let t15299 = t11529 * t1709;
    let t15300 = t1174 * t15299;
    let t15338 = t11588 * t1714;
    let t15363 = t698 * t1716;
    let t15364 = t1174 * t15363;
    let t15376 = t1420 * t337 * t1887;
    (t15281, t15299, t15300, t15338, t15363, t15364, t15376)
}
