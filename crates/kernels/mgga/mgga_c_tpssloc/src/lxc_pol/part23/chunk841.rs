//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 841/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk841<F: Float>(t15908: F, t2375: F, t1787: F, t2516: F, t17: F, t2663: F, t5157: F, t1788: F, t2225: F, t2221: F, t2223: F, t12248: F, t68: F, t544: F, t12189: F, t1804: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15909 = t15908 * t2375;
    let t15971 = t1787 * t2516;
    let t15972 = t17 * t15971;
    let t15979 = t5157 * t2663;
    let t15982 = t2225 * t1788;
    let t15984 = t2221 * t1788;
    let t15986 = t2223 * t1788;
    let t16046 = t68 * t12248;
    let t16047 = t544 * t16046;
    let t16078 = t12189 * t1804;
    (t15909, t15971, t15972, t15979, t15982, t15984, t15986, t16047, t16078)
}
