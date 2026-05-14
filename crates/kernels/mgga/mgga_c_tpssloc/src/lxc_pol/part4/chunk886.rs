//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 886/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk886<F: Float>(t5151: F, t750: F, t17: F, t1787: F, t2516: F, t12120: F, t2663: F, t5157: F, t1788: F, t2225: F, t2221: F, t225: F, t5213: F, t5211: F, t12248: F, t68: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15921 = t5151 * t750;
    let t15923 = 2.0 * t17 * t15921;
    let t15971 = t1787 * t2516;
    let t15972 = t17 * t15971;
    let t15976 = 4.0 * t12120;
    let t15979 = t5157 * t2663;
    let t15982 = t2225 * t1788;
    let t15984 = t2221 * t1788;
    let t16022 = t5213 * t225;
    let t16030 = t5211 * t225;
    let t16046 = t68 * t12248;
    (t15923, t15972, t15976, t15979, t15982, t15984, t16022, t16030, t16046)
}
