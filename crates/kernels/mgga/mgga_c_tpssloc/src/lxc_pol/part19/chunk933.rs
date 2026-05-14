//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 933/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk933<F: Float>(t11203: F, t11161: F, t11170: F, t11197: F, t11200: F, t11206: F, t11209: F, t11211: F, t11213: F, t11215: F, t11217: F, t11221: F, t11224: F, t11314: F, t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11165: F, t11174: F, t11230: F, t11233: F, t11245: F, t11259: F, t11261: F, t11266: F) -> (F, F) {
    let t11317 = 0.46308888888888888888e0 * t11203;
    let t11328 = -t11314 - 0.52945875e1 * t11197 + 0.94674375e0 * t11200 - t11317 + 0.62517e0 * t11206 + 0.104195e0 * t11209 + 0.34731666666666666667e0 * t11211 + 0.69463333333333333335e-1 * t11213 - 0.41678000000000000001e0 * t11215 - 0.20839e0 * t11217 + 0.46308888888888888889e-1 * t11221 - 0.20839e0 * t11224 - 0.103295e1 * t11161 + 0.309885e1 * t11170;
    let t11343 = -0.104195e0 * t11230 + 0.62517e0 * t11233 + 0.68863333333333333332e0 * t11137 + 0.34431666666666666666e0 * t11139 - 0.103295e1 * t11141 - 0.51647499999999999999e0 * t11143 + 0.57386111111111111112e0 * t11150 - 0.20659e1 * t11156 + 0.309885e1 * t11165 + 0.516475e0 * t11174 - 0.157790625e0 * t11245 + 0.3529725e1 * t11259 + 0.6311625e0 * t11261 + 0.264729375e1 * t11266;
    (t11328, t11343)
}
