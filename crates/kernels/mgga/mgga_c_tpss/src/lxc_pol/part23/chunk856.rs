//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 856/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk856<F: Float>(t1281: F, t1904: F, t548: F, t5771: F, t5775: F, t5778: F, t6067: F, t3418: F, t38: F, t1317: F, t84: F, t77: F, t1290: F, t578: F, t1313: F, t76: F) -> (F, F, F, F, F, F) {
    let t6071 = 3.0 * t1281 * t1904 + t548 * t6067 + t5771 + t5775 + t5778;
    let t6073 = t3418 * t38;
    let t6076 = t84 * t1317;
    let t6077 = t77 * t6076;
    let t6080 = t578 * t1290;
    let t6090 = t76 * t1313;
    (t6071, t6073, t6076, t6077, t6080, t6090)
}
