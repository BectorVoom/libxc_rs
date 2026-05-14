//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1272/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1272<F: Float>(t12463: F, t19090: F, t12478: F, t6013: F, t12407: F, t19077: F, t12359: F, t20831: F, t3062: F, t12445: F, t6007: F, t139: F, t20808: F, t3032: F, t4047: F, t20837: F, t3092: F) -> (F, F, F, F, F, F, F, F) {
    let t68365 = t19090 * t12463 / 1152.0;
    let t68373 = t6013 * t12478 / 864.0;
    let t68387 = t19077 * t12407 / 576.0;
    let t68391 = t6013 * t12359 / 1728.0;
    let t68393 = t20831 * t3062 / 216.0;
    let t68394 = t6007 * t12445;
    let t68405 = t20808 * t139 * t3032 * t4047 / 324.0;
    let t68407 = t20837 * t3092 / 324.0;
    (t68365, t68373, t68387, t68391, t68393, t68394, t68405, t68407)
}
