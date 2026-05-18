//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1130/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1130<F: Float>(t4571: F, t6765: F, t4630: F, t6755: F, t1036: F, t7586: F, t1409: F, t1933: F, t1937: F, t1597: F, t40: F, t23479: F) -> (F, F, F, F, F, F) {
    let t25616 = t6765 * t4571;
    let t25618 = t6755 * t4630;
    let t25625 = t7586 * t1036;
    let t25628 = t1933 * t1409;
    let t25629 = t25628 * t1937;
    let t25637 = t40 * t1597;
    let t25638 = t1933 * t25637;
    let t25639 = t25638 * t23479;
    (t25616, t25618, t25625, t25629, t25637, t25639)
}
