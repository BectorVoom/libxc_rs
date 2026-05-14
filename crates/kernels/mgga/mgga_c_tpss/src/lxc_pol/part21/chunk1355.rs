//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1355/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1355<F: Float>(t10456: F, t6113: F, t19441: F, t2056: F, t1659: F, t3245: F, t19620: F, t7029: F, t19434: F, t13235: F, t6106: F, t19327: F, t3499: F, t19626: F, t61801: F, t19577: F, t5755: F) -> (F, F, F, F, F, F, F, F) {
    let t65921 = 4.0 * t10456 * t6113;
    let t65923 = 4.0 * t2056 * t19441;
    let t65924 = t1659 * t3245;
    let t65927 = 6.0 * t19620 * t7029 * t65924;
    let t65929 = 4.0 * t2056 * t19434;
    let t65931 = 2.0 * t13235 * t6106;
    let t65933 = 4.0 * t3499 * t19327;
    let t65935 = 6.0 * t61801 * t19626;
    let t65937 = 2.0 * t19577 * t5755;
    (t65921, t65923, t65927, t65929, t65931, t65933, t65935, t65937)
}
