//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1085/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1085<F: Float>(t1025: F, t15291: F, t1032: F, t5104: F, t673: F, t5085: F, t9271: F, t1027: F, t4079: F, t4087: F, t2885: F, t5092: F, t9267: F, t4071: F, t2868: F, t15248: F, t15251: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15292 = t1025 * t15291;
    let t15294 = t1032 * t15291;
    let t15296 = t673 * t5104;
    let t15298 = t9271 * t5085;
    let t15299 = t15298 * t1027;
    let t15301 = t4087 * t4079;
    let t15303 = t2885 * t5092;
    let t15304 = t15303 * t1027;
    let t15306 = t9267 * t5085;
    let t15307 = t15306 * t1027;
    let t15309 = t4071 * t4079;
    let t15311 = t2868 * t5092;
    let t15312 = t15311 * t1027;
    let t15314 = -0.5519e-1 * t15248 + 0.301925e0 * t15251 + 0.258925e1 * t15292 + 0.16504875e0 * t15294 + 0.18396666666666666667e-1 * t15296 - 0.412621875e-1 * t15299 + 0.16504875e0 * t15301 + 0.82524375e-1 * t15304 + 0.19419375e1 * t15307 - 0.258925e1 * t15309 - 0.1294625e1 * t15312;
    (t15292, t15294, t15296, t15299, t15301, t15304, t15307, t15309, t15312, t15314)
}
