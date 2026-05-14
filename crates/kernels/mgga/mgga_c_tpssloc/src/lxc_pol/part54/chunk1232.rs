//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1232/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1232<F: Float>(t22986: F, t31338: F, t86873: F, t10109: F, t8562: F, t33422: F, t6547: F, t118877: F, t118886: F, t13042: F, t2054: F, t25168: F, t25233: F, t259: F, t2597: F, t26703: F, t31409: F, t33395: F, t33405: F, t33433: F, t4147: F, t4272: F, t6627: F, t7087: F, t798: F, t8563: F, t87755: F, t87810: F) -> (F,) {
    let t121648 = t22986 * t86873 * t31338;
    let t121652 = t10109 * t8562;
    let t121660 = t6547 * t33422;
    let t121668 = 2.0 * t2597 * t33433 + t118877 + 0.16449340668482264365e-1 * t121648 + 2.0 * t7087 * t25233 - 6.0 * t25168 * t121652 * t4272 - t87810 * t2054 + t798 * t33395 * t259 - t13042 * t8563 + 0.19190897446562641759e-1 * t121660 - 6.0 * t87755 * t33405 + t118886 + 2.0 * t4147 * t31409 + 2.0 * t6627 * t26703;
    (t121668,)
}
