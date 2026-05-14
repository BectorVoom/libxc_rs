//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1107/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1107<F: Float>(t1113: F, t1501: F, t12387: F, t3068: F, t12378: F, t1289: F, t12377: F, t1562: F, t4052: F, t4047: F, t9702: F, t1114: F, t5068: F, t5072: F, t1101: F, t13335: F) -> (F, F, F, F, F, F, F) {
    let t15567 = t1501 * t1113;
    let t15568 = t12387 * t15567;
    let t15569 = t3068 * t15568;
    let t15572 = t12378 * t1289;
    let t15573 = t12377 * t15572;
    let t15574 = t3068 * t15573;
    let t15577 = t1562 * t4052;
    let t15578 = t3068 * t15577;
    let t15581 = t1562 * t4047;
    let t15582 = t9702 * t15581;
    let t15585 = t5068 * t1114;
    let t15586 = t3068 * t15585;
    let t15589 = t5072 * t1114;
    let t15590 = t3068 * t15589;
    let t15595 = t1101 * t13335;
    (t15569, t15574, t15578, t15582, t15586, t15590, t15595)
}
