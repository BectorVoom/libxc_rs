//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1420/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1420<F: Float>(t22530: F, t645: F, t72: F, t1864: F, t2307: F, t1863: F, t22522: F, t9239: F, t2241: F, t641: F, t608: F, t9228: F) -> (F, F, F, F, F) {
    let t83734 = t72 * t22530 * t645;
    let t83737 = t1864 * t2307;
    let t83738 = t1863 * t83737;
    let t83741 = t9239 * t22522;
    let t83745 = t72 * t641 * t2241;
    let t83748 = t9228 * t608;
    (t83734, t83738, t83741, t83745, t83748)
}
