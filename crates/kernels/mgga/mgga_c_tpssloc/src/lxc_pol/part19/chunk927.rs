//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 927/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk927<F: Float>(t11159: F, t3297: F, t136: F, t1113: F, t11168: F, t407: F, t1102: F, t3271: F, t11135: F, t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11161: F, t11165: F, t11170: F, t11174: F) -> (F, F, F, F, F, F, F, F) {
    let t11229 = t3297 * t11159;
    let t11230 = t136 * t11229;
    let t11232 = t1113 * t11168;
    let t11233 = t136 * t11232;
    let t11243 = 1.0/pow_3_2(t407);
    let t11244 = t3271 * t1102;
    let t11245 = t11243 * t11244;
    let t11247 = 28.0 / 27.0 * t11135;
    let t11258 = -t11247 + 4.0 / 9.0 * t11137 + 2.0 / 9.0 * t11139 - 2.0 / 3.0 * t11141 - t11143 / 3.0 + 10.0 / 27.0 * t11150 - 4.0 / 3.0 * t11156 - 2.0 / 3.0 * t11161 + 2.0 * t11165 + 2.0 * t11170 + t11174 / 3.0;
    (t11229, t11230, t11232, t11233, t11243, t11244, t11245, t11258)
}
