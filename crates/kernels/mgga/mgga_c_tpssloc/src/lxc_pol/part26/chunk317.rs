//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 317/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk317<F: Float>(t1100: F, t1102: F, t1086: F, t407: F, t281: F, t415: F, t904: F, t241: F, t457: F) -> (F, F, F, F, F, F, F) {
    let t1103 = t1100 * t1102;
    let t1105 = 0.29896666666666666667e0 * t1086;
    let t1107 = f64::sqrt(t407);
    let t1108 = t1107 * t1102;
    let t1111 = t281 * t904 * t415;
    let t1112 = 0.82156666666666666667e-1 * t1111;
    let t1113 = t241 * t457;
    (t1103, t1105, t1107, t1108, t1111, t1112, t1113)
}
