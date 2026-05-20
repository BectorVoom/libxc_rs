//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1256/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1256<F: Float>(t1352: F, t19956: F, t5248: F, t5250: F, t5249: F, t5287: F, t19871: F, t120: F, t6330: F, t12419: F, t6347: F, t3805: F) -> (F, F, F, F, F, F) {
    let t19962 = t5248 * t19956 * t1352;
    let t19966 = t5248 * t19956 * t5250;
    let t19972 = t5248 * t5249 * t5287;
    let t19976 = t5248 * t19871 * t1352;
    let t19979 = t120 * t6330;
    let t19981 = t12419 * t19979 * t1352;
    let t19984 = t120 * t6347;
    let t19986 = t3805 * t19984 * t1352;
    (t19962, t19966, t19972, t19976, t19981, t19986)
}
