//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2176/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2176<F: Float>(t19572: F, t67: F, t758: F, t2221: F, t6328: F, t2225: F, t17: F, t2516: F, t6320: F, t750: F, t19644: F, t225: F) -> (F, F, F, F, F, F) {
    let t56374 = t19572 * t67 * t758;
    let t56390 = t2221 * t6328;
    let t56394 = t2225 * t6328;
    let t56398 = t17 * t6320 * t2516;
    let t56400 = t17 * t19572 * t750;
    let t56422 = t19644 * t225;
    (t56374, t56390, t56394, t56398, t56400, t56422)
}
