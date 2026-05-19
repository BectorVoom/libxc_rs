//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 242/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk242<F: Float>(t1102: F, t1107: F, t281: F, t415: F, t904: F, t241: F, t457: F, t1090: F, t136: F, t1092: F, t1103: F, t1105: F) -> (F, F, F, F, F, F) {
    let t1108 = t1107 * t1102;
    let t1111 = t281 * t904 * t415;
    let t1112 = F::cast_from(0.82156666666666666667e-1_f64) * t1111;
    let t1113 = t241 * t457;
    let t1114 = t1113 * t1090;
    let t1115 = t136 * t1114;
    let t1117 = F::new(0.1898925e1) * t1103 - t1105 + F::cast_from(0.29896666666666666667e0_f64) * t1092 + F::new(0.3071625e0) * t1108 - t1112 + F::cast_from(0.82156666666666666667e-1_f64) * t1115;
    (t1108, t1111, t1113, t1114, t1115, t1117)
}
