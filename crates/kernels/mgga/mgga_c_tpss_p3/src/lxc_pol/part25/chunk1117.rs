//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1117/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1117<F: Float>(t11850: F, t11875: F, t11932: F, t12093: F, t12104: F, t12115: F, t15237: F, t15239: F, t15241: F, t15243: F, t15245: F, t15259: F, t15314: F, t15321: F, t15324: F, t15327: F, t15330: F, t15349: F, t9182: F, t9192: F, t9214: F, t9221: F) -> F {
    let t15351 = -t9182 + F::cast_from(0.91983333333333333333e-1_f64) * t9192 - t12093 + F::cast_from(0.18396666666666666667e0_f64) * t11850 - t9214 + F::cast_from(0.82785e-1_f64) * t15237 + F::cast_from(0.67094444444444444443e-1_f64) * t15239 - F::cast_from(0.20128333333333333333e0_f64) * t15241 - F::cast_from(0.10064166666666666667e0_f64) * t15243 - F::cast_from(0.11038e0_f64) * t15245 + t15314 + t12104 - F::cast_from(0.40256666666666666668e0_f64) * t11875 - t12115 + F::cast_from(0.13418888888888888889e0_f64) * t9221 + F::cast_from(0.36793333333333333333e-1_f64) * t11932 + F::cast_from(0.33547222222222222222e0_f64) * t15259 + F::cast_from(0.33114e0_f64) * t15321 + F::cast_from(0.49671e0_f64) * t15324 + F::cast_from(0.16557e0_f64) * t15327 - F::cast_from(0.5519e-1_f64) * t15330 + t15349;
    t15351
}
