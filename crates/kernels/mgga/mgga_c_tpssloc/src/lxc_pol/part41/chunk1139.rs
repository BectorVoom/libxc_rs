//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1139/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1139<F: Float>(t120: F, t6330: F, t12419: F, t1352: F, t6347: F, t3805: F, t5187: F, t550: F, t5249: F, t1307: F, t3870: F, t820: F, t19744: F, t19871: F, t5248: F, t12369: F) -> (F, F, F, F, F, F) {
    let t19979 = t120 * t6330;
    let t19981 = t12419 * t19979 * t1352;
    let t19984 = t120 * t6347;
    let t19986 = t3805 * t19984 * t1352;
    let t19989 = t550 * t5187;
    let t19991 = t3805 * t5249 * t19989;
    let t19994 = t6347 * t1307;
    let t19996 = t3870 * t820 * t19994;
    let t20000 = t5248 * t19871 * t19744;
    let t20004 = t3805 * t19871 * t12369;
    (t19981, t19986, t19991, t19996, t20000, t20004)
}
