//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 948/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk948<F: Float>(t1184: F, t3305: F, t1193: F, t8027: F, t1170: F, t3280: F, t8017: F, t1220: F, t2376: F, t339: F, t1235: F, t3256: F, t789: F, t3263: F, t1218: F, t230: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10033 = t3305 * t1184;
    let t10038 = 0.35089341735807877242e1 * t1193 * t8027;
    let t10039 = t1170 * t3280;
    let t10042 = 0.5848223622634646207e0 * t1193 * t8017;
    let t10077 = t339 * t1220 * t2376;
    let t10078 = t10077 * t1235;
    let t10081 = t339 * t3256 * t789;
    let t10082 = t10081 * t3263;
    let t10084 = t1218 * t1218;
    let t10085 = 1.0 / t10084;
    let t10086 = t10085 * t230;
    (t10033, t10038, t10039, t10042, t10077, t10078, t10082, t10085, t10086)
}
