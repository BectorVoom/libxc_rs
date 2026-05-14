//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1110/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1110<F: Float>(t1177: F, t14753: F, t14744: F, t1011: F, t15031: F, t1212: F, t1226: F, t4965: F, t11652: F, t11665: F, t11678: F, t11692: F, t11699: F, t11703: F, t1174: F, t1218: F, t1232: F, t15560: F, t15564: F, t15569: F, t15574: F, t15580: F, t15581: F, t3496: F, t3580: F, t4950: F, t5002: F) -> (F,) {
    let t15584 = t1177 * t14753;
    let t15587 = t1177 * t14744;
    let t15590 = t15031 * t1011;
    let t15591 = t15590 * t1212;
    let t15594 = t4965 * t1226;
    let t15601 = -t11678 * t15560 / 2304.0 + t11692 * t15564 / 4608.0 + t15569 * t3580 / 432.0 - t15574 - t11665 * t4950 / 2304.0 - t11652 / 4608.0 - t15580 - t1174 * t15581 / 72.0 - t1174 * t15584 / 144.0 - t1174 * t15587 / 48.0 + t15591 * t1218 / 1536.0 - t15594 * t1232 / 2304.0 + t5002 * t3496 / 3072.0 - t11699 / 3456.0 + t11703 / 4608.0;
    (t15601,)
}
