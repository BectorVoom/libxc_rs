//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1726/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1726<F: Float>(t22530: F, t72: F, t2244: F, t605: F, t2251: F, t6489: F, t9239: F, t2241: F, t79: F, t2240: F, t608: F, t1864: F, t645: F) -> (F, F, F, F, F, F, F) {
    let t22531 = t72 * t22530;
    let t22534 = t605 * t2244;
    let t22537 = t605 * t2251;
    let t22544 = t9239 * t6489;
    let t22546 = t72 * t79 * t2241;
    let t22549 = t2240 * t608;
    let t22550 = t1864 * t645;
    (t22531, t22534, t22537, t22544, t22546, t22549, t22550)
}
