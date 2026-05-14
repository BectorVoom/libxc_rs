//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1026/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1026<F: Float>(t27982: F, t7032: F, t26959: F, t7435: F, t7432: F, t91957: F, t27966: F, t1409: F, t605: F, t63: F, t27961: F, t84219: F, t55921: F, t7025: F, t2240: F, t5392: F) -> (F, F, F, F, F, F, F, F) {
    let t102215 = t27982 * t7032;
    let t102217 = t7435 * t26959;
    let t102219 = t91957 * t7432;
    let t102221 = t27966 * t7032;
    let t102227 = t605 * t1409 * t63;
    let t102248 = t84219 * t27961;
    let t102267 = t55921 * t7025;
    let t102275 = t2240 * t5392 * t63;
    (t102215, t102217, t102219, t102221, t102227, t102248, t102267, t102275)
}
