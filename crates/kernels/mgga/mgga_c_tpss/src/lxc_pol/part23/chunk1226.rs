//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1226/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1226<F: Float>(t4275: F, t6013: F, t1116: F, t1130: F, t19080: F, t19084: F, t19090: F, t19094: F, t19095: F, t20831: F, t20834: F, t20837: F, t4253: F, t4271: F, t4280: F, t4285: F, t4289: F) -> (F,) {
    let t20844 = t6013 * t4275;
    let t20852 = -t19090 * t4253 / 1536.0 - t20831 * t1116 / 288.0 - t20834 / 432.0 + t20837 * t1130 / 432.0 + t19080 / 2304.0 - t19094 - t19095 / 3456.0 - t19084 * t4271 / 2304.0 - t20844 / 3456.0 + 5.0 / 6912.0 * t6013 * t4280 - t6013 * t4285 / 1152.0 - t6013 * t4289 / 2304.0;
    (t20852,)
}
