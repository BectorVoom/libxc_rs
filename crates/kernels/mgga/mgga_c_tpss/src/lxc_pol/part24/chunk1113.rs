//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1113/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1113<F: Float>(t15854: F, t3068: F, t1125: F, t12435: F, t12446: F, t12448: F, t12472: F, t15828: F, t15832: F, t15835: F, t15839: F, t15843: F, t15848: F, t3052: F, t3080: F, t4253: F, t4271: F, t9573: F, t9626: F) -> (F,) {
    let t15855 = t3068 * t15854;
    let t15860 = -t1125 * t15828 / 1152.0 - t12446 / 6912.0 + t15832 / 162.0 - t15835 / 864.0 + t3052 * t15839 / 768.0 - t3080 * t15843 / 1536.0 - t9626 * t15848 / 512.0 + t12448 / 1296.0 + t12435 * t4253 / 288.0 + t9573 * t15855 / 4608.0 + t12472 * t4271 / 432.0;
    (t15860,)
}
