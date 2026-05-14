//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 980/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk980<F: Float>(t2251: F, t605: F, t2241: F, t72: F, t79: F, t2240: F, t608: F, t1864: F, t645: F, t1266: F, t6534: F, t652: F, t192: F, t532: F, t1982: F) -> (F, F, F, F, F, F, F, F) {
    let t22537 = t605 * t2251;
    let t22546 = t72 * t79 * t2241;
    let t22549 = t2240 * t608;
    let t22550 = t1864 * t645;
    let t22561 = t1266 * t6534;
    let t22563 = 4.0 * t652 * t22561;
    let t22573 = t192 * t532;
    let t22574 = t1982 * t22573;
    (t22537, t22546, t22549, t22550, t22561, t22563, t22573, t22574)
}
