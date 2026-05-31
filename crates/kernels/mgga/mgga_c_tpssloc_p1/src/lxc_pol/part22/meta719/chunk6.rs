//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2333/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2333<F: Float>(t20974: F, t9638: F, t20891: F, t120: F, t20800: F, t20904: F, t41414: F, t13177: F, t13251: F, t16673: F, t16898: F, t20756: F, t2643: F, t2645: F, t40966: F, t40971: F, t4177: F, t4184: F, t4250: F, t46546: F, t5619: F, t58421: F, t58425: F, t58427: F, t58642: F, t776: F, t820: F, t829: F, t843: F) -> F {
    let t67637 = t9638 * t20974;
    let t67639 = t9638 * t20891;
    let t67644 = t120 * t20800;
    let t67660 = t41414 * t20904;
    let t67667 = F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t67637 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t67639 + t16673 * t4177 * t4184 / F::cast_from(512.0_f64) + t2643 * t2645 * t67644 * t829 / F::cast_from(768.0_f64) - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t13251 * t16898 + t58642 * t4250 / F::cast_from(256.0_f64) + F::cast_from(595.0_f64) / F::cast_from(2592.0_f64) * t40966 + F::cast_from(455.0_f64) / F::cast_from(216.0_f64) * t46546 + F::cast_from(119.0_f64) / F::cast_from(576.0_f64) * t58421 + F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t58425 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t58427 - t13177 * t5619 / F::cast_from(1024.0_f64) + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t67660 + F::cast_from(35.0_f64) / F::cast_from(128.0_f64) * t843 * t40971 * t820 * t20756 * t776;
    t67667
}
