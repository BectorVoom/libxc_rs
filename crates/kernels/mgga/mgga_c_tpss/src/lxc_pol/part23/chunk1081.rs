//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1081/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1081<F: Float>(t11476: F, t12399: F, t3931: F, t11453: F, t4279: F, t1125: F, t4233: F, t3052: F, t1501: F, t3081: F, t3068: F, t1562: F, t2841: F, t9702: F, t1111: F, t12330: F, t12355: F, t12361: F, t12363: F, t12368: F, t12371: F, t12374: F, t12381: F, t12385: F, t12391: F, t12395: F, t3067: F, t9556: F, t9563: F, t9573: F, t9633: F, t9658: F, t9661: F) -> (F, F, F, F, F, F) {
    let t12400 = t12399 * t11476;
    let t12401 = t3931 * t12400;
    let t12404 = t11453 * t4279;
    let t12406 = 5.0 / 10368.0 * t1125 * t12404;
    let t12407 = t11453 * t4233;
    let t12409 = t3052 * t12407 / 1152.0;
    let t12410 = t1501 * t3081;
    let t12411 = t3068 * t12410;
    let t12414 = t1562 * t2841;
    let t12415 = t9702 * t12414;
    let t12421 = -t9556 * t12330 / 2304.0 + t1111 * t12355 / 3072.0 - t12361 - t1125 * t12363 / 4608.0 + t12368 / 20736.0 - t12371 - t3067 * t12374 / 1152.0 + t9573 * t12381 / 2304.0 + t12385 / 1296.0 - t9556 * t12391 / 1152.0 + 5.0 / 6912.0 * t3067 * t12395 - t9563 / 3456.0 - t1125 * t12401 / 768.0 + t12406 + t12409 + t9573 * t12411 / 4608.0 + 5.0 / 13824.0 * t3067 * t12415 + t9633 / 648.0 + t9658 / 648.0 - t9661 / 864.0;
    (t12401, t12404, t12407, t12411, t12415, t12421)
}
