//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1227/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1227<F: Float>(t1653: F, t3509: F, t3578: F, t3516: F, t1742: F, t478: F, t3068: F, t1244: F, t11697: F, t4949: F, t3577: F, t3431: F, t4729: F) -> (F, F, F, F, F) {
    let t15559 = t1653 * t3509;
    let t15560 = t3578 * t15559;
    let t15563 = t1653 * t3516;
    let t15564 = t3578 * t15563;
    let t15567 = t478 * t1742;
    let t15568 = t15567 * t3068;
    let t15569 = t1244 * t15568;
    let t15572 = t11697 * t4949;
    let t15574 = t3577 * t15572 / F::new(3456.0);
    let t15578 = t3431 * t4729;
    (t15560, t15564, t15569, t15574, t15578)
}
