//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2270/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2270<F: Float>(t1625: F, t7577: F, t14552: F, t1604: F, t17691: F, t23327: F, t23329: F, t254: F, t25423: F, t25424: F, t25429: F, t25431: F, t25442: F, t25750: F, t25759: F, t25801: F, t25815: F, t28701: F, t4342: F, t6691: F, t7553: F, t7625: F, t82502: F, t88050: F, t88058: F, t88096: F, t88112: F, t88162: F, t99070: F) -> F {
    let t99131 = t7577 * t1625;
    let t99143 = -F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t25442 * t25801 + F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t88112 * t4342 * t99070 + F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t82502 * t28701 + F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t88162 * t25424 - F::cast_from(0.73108180748810063845e-2_f64) * t25429 * t88162 * t25431 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t88058 * t7553 + F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t88162 * t25815 - F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t23329 * t25423 * t17691 - F::cast_from(0.54831135561607547883e-2_f64) * t23327 * t99131 * t6691 - F::cast_from(0.54831135561607547883e-2_f64) * t23327 * t88050 * t25750 + t88096 - F::cast_from(12.0_f64) * t1604 * t254 * t25759 - F::cast_from(2.0_f64) * t14552 * t7625;
    t99143
}
