//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1020/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1020<F: Float>(t11588: F, t2127: F, t221: F, t82631: F, t2122: F, t7319: F, t1235: F, t225: F, t461: F, t25: F, t40772: F, t1519: F, t213: F, t794: F, t25051: F, t1509: F, t6624: F) -> (F, F, F, F, F, F, F, F, F) {
    let t85639 = t2127 * t221 * t11588;
    let t85660 = t2127 * t82631;
    let t86403 = t7319 * t2122;
    let t86415 = t461 * t1235 * t225;
    let t86716 = t40772 * t25;
    let t86873 = t213 * t1519 * t225;
    let t86893 = t794 * t1519;
    let t86988 = t25051 * t225;
    let t87567 = t6624 * t1509;
    (t85639, t85660, t86403, t86415, t86716, t86873, t86893, t86988, t87567)
}
