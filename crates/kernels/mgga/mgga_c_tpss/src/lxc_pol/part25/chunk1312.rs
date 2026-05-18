//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1312/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1312<F: Float>(t18464: F, t5420: F, t5424: F, t13771: F, t5728: F, t13793: F, t215: F, t65595: F, t13798: F, t19469: F, t19539: F, t6259: F) -> (F, F, F, F, F, F) {
    let t69553 = t18464 * t5420;
    let t69555 = t18464 * t5424;
    let t69558 = t5728 * t13771;
    let t69561 = t65595 * t215 * t13793;
    let t69564 = t19469 * t215 * t13798;
    let t69654 = t6259 * t19539;
    (t69553, t69555, t69558, t69561, t69564, t69654)
}
