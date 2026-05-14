//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1019/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1019<F: Float>(t136: F, t18507: F, t1113: F, t18225: F, t6017: F, t699: F, t18232: F, t3297: F, t18237: F, t18241: F, t11211: F, t11487: F, t14766: F, t15347: F, t15348: F, t15349: F, t18494: F, t18497: F, t18500: F, t18503: F, t18505: F) -> (F, F, F, F, F, F, F) {
    let t18508 = t136 * t18507;
    let t18509 = t1113 * t18225;
    let t18510 = t136 * t18509;
    let t18512 = t699 * t6017;
    let t18514 = t3297 * t18232;
    let t18515 = t136 * t18514;
    let t18517 = t1113 * t18237;
    let t18518 = t136 * t18517;
    let t18520 = t1113 * t18241;
    let t18521 = t136 * t18520;
    let t18523 = t11487 - 5.0 / 27.0 * t11211 - 10.0 / 27.0 * t14766 - t15347 + t15348 + t15349 - t18494 / 27.0 - 2.0 / 27.0 * t18497 + t18500 / 3.0 + t18503 / 9.0 + 2.0 / 9.0 * t18505 - t18508 - 2.0 / 3.0 * t18510 + t18512 / 9.0 + t18515 / 18.0 - t18518 / 3.0 - t18521 / 6.0;
    (t18508, t18510, t18512, t18515, t18518, t18521, t18523)
}
