//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 927/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk927<F: Float>(t114472: F, t114483: F, t114489: F, t114494: F, t114500: F, t115983: F, t115984: F, t115990: F, t115995: F, t115996: F, t116000: F, t116004: F, t116006: F, t116008: F, t2039: F, t2319: F, t23877: F, t23880: F, t24481: F, t671: F, t7056: F, t84004: F, t91803: F) -> (F,) {
    let t116011 = t114472 + t115983 + 27.0 * t115984 * t2319 + 0.135e2 * t84004 * t2039 + t115990 + t114483 + 27.0 * t23880 * t24481 + t114489 + t115995 + t114494 + 27.0 * t115996 * t671 + t116000 + t114500 + 27.0 * t23877 * t7056 + t116004 + t116006 + t116008 + 27.0 * t91803 * t2039;
    (t116011,)
}
