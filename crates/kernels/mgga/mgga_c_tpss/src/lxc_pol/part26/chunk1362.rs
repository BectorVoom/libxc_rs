//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1362/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1362<F: Float>(t5248: F, t6032: F, t19144: F, t1586: F, t20913: F, t1148: F, t1561: F, t15956: F, t15964: F, t15968: F, t15999: F, t16004: F, t16012: F, t1880: F, t19123: F, t19129: F, t19143: F, t20856: F, t20865: F, t20874: F, t20877: F, t20891: F, t20892: F, t20896: F, t20903: F, t22046: F, t22061: F, t3126: F, t3139: F, t4303: F, t4314: F, t4322: F, t4323: F, t5242: F, t6024: F, t6025: F, t6034: F, t6038: F, t63200: F, t63357: F, t63383: F, t6509: F, t68278: F, t68557: F, t73264: F, t73285: F, t73289: F) -> (F,) {
    let t73481 = t6032 * t5248;
    let t73508 = t19144 * t5248;
    let t73532 = t20913 * t1586;
    let t73540 = -2.0 * t19143 * t19144 * t5242 * t4303 + t19143 * t73289 * t4314 - 2.0 * t63357 * t73481 * t3139 * t1148 + 2.0 * t19143 * t20891 * t15999 - t63200 * t73481 * t16004 - 4.0 * t19143 * t68557 * t1561 * t4303 - 12.0 * t20865 * t20874 + 8.0 * t68278 * t73264 * t4303 + 2.0 * t6024 * t6025 * t1880 * t16012 - 12.0 * t6024 * t19123 * t22061 * t1148 + 4.0 * t63357 * t73508 * t3126 * t1148 - 4.0 * t19143 * t20896 * t15964 - 6.0 * t63200 * t63383 * t5248 * t15956 + 6.0 * t63200 * t73508 * t15968 + 4.0 * t19129 * t73285 * t20892 + 4.0 * t19129 * t20877 * t20903 - 2.0 * t20856 * t4323 - t22046 * t6038 + 4.0 * t19129 * t73532 * t6034 + 4.0 * t6024 * t6025 * t6509 * t4322;
    (t73540,)
}
