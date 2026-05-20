//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1168/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1168<F: Float>(t136: F, t18499: F, t18215: F, t3297: F, t6014: F, t699: F, t1113: F, t18221: F, t18225: F, t6017: F, t18232: F, t18237: F) -> (F, F, F, F, F, F, F, F) {
    let t18500 = t136 * t18499;
    let t18502 = t3297 * t18215;
    let t18503 = t136 * t18502;
    let t18505 = t699 * t6014;
    let t18507 = t1113 * t18221;
    let t18508 = t136 * t18507;
    let t18509 = t1113 * t18225;
    let t18510 = t136 * t18509;
    let t18512 = t699 * t6017;
    let t18514 = t3297 * t18232;
    let t18515 = t136 * t18514;
    let t18517 = t1113 * t18237;
    (t18500, t18503, t18505, t18508, t18510, t18512, t18515, t18517)
}
