//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1298/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1298<F: Float>(t18464: F, t5424: F, t13771: F, t5728: F, t13793: F, t215: F, t65595: F, t13798: F, t19469: F, t60731: F, t60750: F, t65629: F, t65640: F, t65644: F, t67173: F, t67187: F, t69551: F, t69553: F) -> (F,) {
    let t69555 = t18464 * t5424;
    let t69558 = t5728 * t13771;
    let t69561 = t65595 * t215 * t13793;
    let t69564 = t19469 * t215 * t13798;
    let t69567 = -t69551 / 1536.0 - 35.0 / 576.0 * t69553 + 7.0 / 576.0 * t69555 - t67173 + t65629 - 35.0 / 216.0 * t60731 - t69558 / 384.0 - t69561 / 4.0 + t69564 / 8.0 - t65640 + t65644 - t67187 - 119.0 / 1728.0 * t60750;
    (t69567,)
}
