//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1096/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1096<F: Float>(t14953: F, t14997: F, t15056: F, t15115: F, t219: F, t5013: F, t5017: F, t9067: F, t990: F, t1482: F, t2776: F, t4016: F) -> (F, F, F, F, F) {
    let t15117 = t14953 + t14997 + t15056 + t15115;
    let t15118 = param_beta * t15117;
    let t15120 = t5013 * t219;
    let t15131 = t9067 * t5017 * t990;
    let t15135 = t2776 * t1482 * t4016;
    (t15117, t15118, t15120, t15131, t15135)
}
