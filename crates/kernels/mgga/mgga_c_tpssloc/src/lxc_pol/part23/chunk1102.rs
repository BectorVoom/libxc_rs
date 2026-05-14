//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1102/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1102<F: Float>(t2281: F, t5489: F, t5465: F, t2239: F, t5385: F, t19681: F, t2528: F, t2535: F, t2371: F, t19575: F, t592: F, t2221: F, t6328: F, t2223: F, t2225: F, t17: F, t2516: F, t6320: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t55531 = t2281 * t5489;
    let t55537 = t2281 * t5465;
    let t55921 = t5385 * t2239;
    let t56099 = t19681 * t2528;
    let t56104 = t19681 * t2535;
    let t56168 = t19681 * t2371;
    let t56185 = t592 * t19575;
    let t56390 = t2221 * t6328;
    let t56392 = t2223 * t6328;
    let t56394 = t2225 * t6328;
    let t56398 = t17 * t6320 * t2516;
    (t55531, t55537, t55921, t56099, t56104, t56168, t56185, t56390, t56392, t56394, t56398)
}
