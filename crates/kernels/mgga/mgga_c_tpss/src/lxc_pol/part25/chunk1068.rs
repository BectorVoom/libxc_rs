//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1068/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1068<F: Float>(t1148: F, t5294: F, t3118: F, t1113: F, t9751: F, t1133: F, t5248: F, t3126: F, t4245: F, t9765: F, t1561: F, t4293: F, t1578: F, t5242: F, t15796: F, t466: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15952 = t5294 * t1148;
    let t15953 = t3118 * t15952;
    let t15956 = t9751 * t1113;
    let t15960 = t1133 * t5248;
    let t15964 = t3126 * t4245;
    let t15968 = t9765 * t1113;
    let t15975 = t4293 * t1561;
    let t15979 = t1578 * t4245;
    let t15989 = t1133 * t5242;
    let t15992 = t466 * t15796;
    (t15953, t15956, t15960, t15964, t15968, t15975, t15979, t15989, t15992)
}
