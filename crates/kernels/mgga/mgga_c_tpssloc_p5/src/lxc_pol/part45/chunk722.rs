//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 722/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk722<F: Float>(t2230: F, t6924: F, t213: F, t6928: F, t1998: F, t236: F, t3719: F, t6926: F, t10: F, t2229: F, t60: F, t1995: F) -> (F, F, F, F, F) {
    let t22803 = t2230 * t6924;
    let t22804 = t22803 * t213;
    let t22805 = t22804 * t6928;
    let t22808 = t1998 * t236 * t3719;
    let t22809 = t6926 * t22808;
    let t22811 = t2229 * t10;
    let t22813 = F::cast_from(1.0_f64) / t60 / t22811;
    let t22814 = t22813 * t1995;
    (t22804, t22805, t22809, t22813, t22814)
}
