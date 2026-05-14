//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 868/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk868<F: Float>(t5772: F, t645: F, t547: F, t117: F, t5531: F, t1281: F, t1784: F, t548: F, t5766: F, t5771: F, t3418: F, t38: F, t1317: F, t84: F, t77: F, t1290: F, t578: F) -> (F, F, F, F, F, F, F) {
    let t5773 = t5772 * t645;
    let t5775 = 6.0 * t547 * t5773;
    let t5776 = t117 * t5531;
    let t5778 = 3.0 * t547 * t5776;
    let t5779 = 3.0 * t1281 * t1784 + t548 * t5766 + t5771 + t5775 + t5778;
    let t6073 = t3418 * t38;
    let t6076 = t84 * t1317;
    let t6077 = t77 * t6076;
    let t6080 = t578 * t1290;
    (t5773, t5776, t5779, t6073, t6076, t6077, t6080)
}
