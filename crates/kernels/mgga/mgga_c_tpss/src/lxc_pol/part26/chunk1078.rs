//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1078/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1078<F: Float>(t14953: F, t14997: F, t15056: F, t15115: F, t219: F, t5013: F, t5017: F, t9067: F, t990: F, t1482: F, t2776: F, t4016: F, t5036: F, t9081: F, t948: F, t4977: F, t975: F) -> (F, F, F, F, F, F, F, F) {
    let t15117 = t14953 + t14997 + t15056 + t15115;
    let t15118 = param_beta * t15117;
    let t15120 = t5013 * t219;
    let t15131 = t9067 * t5017 * t990;
    let t15135 = t2776 * t1482 * t4016;
    let t15139 = t5036 * t990;
    let t15140 = t2776 * t15139;
    let t15143 = t9081 * t948;
    let t15147 = t975 * t4977;
    (t15117, t15118, t15120, t15131, t15135, t15140, t15143, t15147)
}
