//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1134/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1134<F: Float>(t111: F, t7415: F, t11588: F, t2127: F, t221: F, t82631: F, t2122: F, t7319: F, t1235: F, t225: F, t461: F, t25: F, t40772: F) -> (F, F, F, F, F, F) {
    let t85416 = t7415 * t111;
    let t85639 = t2127 * t221 * t11588;
    let t85660 = t2127 * t82631;
    let t86403 = t7319 * t2122;
    let t86415 = t461 * t1235 * t225;
    let t86716 = t40772 * t25;
    (t85416, t85639, t85660, t86403, t86415, t86716)
}
