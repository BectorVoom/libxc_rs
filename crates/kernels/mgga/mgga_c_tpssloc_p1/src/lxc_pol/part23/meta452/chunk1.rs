//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1303/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1303<F: Float>(t75916: F, t75928: F, t157: F, t182: F, t58057: F, t1530: F, t193: F, t20756: F, t39529: F, t40779: F, t40784: F, t40790: F, t40793: F, t40797: F, t75894: F, t75895: F, t75900: F, t75901: F, t870: F) -> (F, F, F, F) {
    let t75929 = t75916 + t75928;
    let t75932 = F::cast_from(0.19751673498613801407e-1_f64) * t75929 * t157 * t182;
    let t75933 = F::cast_from(0.70178683471615754484e1_f64) * t58057;
    let t75934 = F::cast_from(24.0_f64) * t1530 * t193 * t20756 * t870 - t39529 - t40779 + t40784 + t40790 + t40793 + t40797 + t75894 + t75895 + t75900 - t75901 + t75932 + t75933;
    (t75929, t75932, t75933, t75934)
}
