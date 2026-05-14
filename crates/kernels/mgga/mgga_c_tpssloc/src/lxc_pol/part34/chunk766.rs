//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 766/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk766<F: Float>(t1807: F, t6434: F, t12351: F, t20356: F, t820: F, t1825: F, t19956: F, t5248: F, t550: F, t6330: F, t12419: F, t5249: F, t19871: F, t3805: F, t6394: F, t6347: F) -> (F, F, F, F, F, F, F) {
    let t20420 = t1807 * t6434;
    let t20433 = t12351 * t820 * t20356;
    let t20442 = t5248 * t19956 * t1825;
    let t20448 = t550 * t6330;
    let t20450 = t12419 * t5249 * t20448;
    let t20454 = t3805 * t19871 * t6394;
    let t20460 = t3805 * t19956 * t6394;
    let t20463 = t550 * t6347;
    (t20420, t20433, t20442, t20450, t20454, t20460, t20463)
}
