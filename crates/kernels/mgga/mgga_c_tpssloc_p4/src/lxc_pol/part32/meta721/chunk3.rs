//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2293/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2293<F: Float>(t27548: F, t8020: F, t29614: F, t491: F, t103218: F, t1653: F, t17691: F, t19225: F, t2155: F, t24589: F, t24590: F, t24601: F, t27388: F, t27426: F, t27433: F, t27445: F, t27446: F, t27549: F, t27774: F, t27776: F, t29690: F, t5059: F, t65203: F, t66822: F, t7283: F, t7287: F, t7288: F, t7300: F, t8002: F, t85674: F, t94374: F, t94378: F, t94395: F, t94514: F) -> (F, F) {
    let t103223 = t8020 * t27548;
    let t103226 = t29614 * t491;
    let t103258 = -F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t27426 * t27388 - F::cast_from(0.26806332941230356743e-1_f64) * t103218 * t7288 + F::cast_from(0.29243272299524025538e-1_f64) * t94395 * t27446 - F::cast_from(0.19495514866349350359e-1_f64) * t103223 * t27776 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t103226 * t7287 + F::cast_from(0.73108180748810063846e-2_f64) * t27549 * t24601 * t27774 * t17691 - F::cast_from(0.36554090374405031923e-2_f64) * t27549 * t24590 * t29690 - F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t94514 * t27433 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t94374 * t8002 + F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t94514 * t27445 - F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t94378 * t1653 * t5059 - F::cast_from(0.49348022005446793095e-1_f64) * t7283 * t7300 * t85674 * t19225 - F::cast_from(2.0_f64) * t66822 * t2155 - F::cast_from(2.0_f64) * t65203 * t2155;
    (t103223, t103258)
}
