//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2197/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2197<F: Float>(t10277: F, t387: F, t1625: F, t225: F, t344: F, t12648: F, t14165: F, t1927: F, t23327: F, t23329: F, t23332: F, t23588: F, t23594: F, t23728: F, t25416: F, t25423: F, t25425: F, t25429: F, t25431: F, t25432: F, t25442: F, t25815: F, t4548: F, t6691: F, t7553: F, t82402: F, t82417: F, t82502: F, t83352: F, t88004: F, t88016: F, t88022: F, t88023: F) -> F {
    let t88035 = t387 * t10277;
    let t88050 = t344 * t1625 * t225;
    let t88054 = -F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t88004 * t6691 + F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t82502 * t25815 + F::cast_from(0.73108180748810063846e-2_f64) * t25429 * t82417 * t25431 + F::cast_from(0.29243272299524025538e-1_f64) * t82402 * t25425 - F::cast_from(0.19495514866349350359e-1_f64) * t88016 * t25432 + F::cast_from(0.14621636149762012769e-1_f64) * t82402 * t25416 + F::cast_from(0.8529287754027840782e-2_f64) * t88022 * t23329 * t88023 * t14165 + F::cast_from(0.16449340668482264365e-1_f64) * t1927 * t4548 * t23588 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23329 * t25423 * t12648 - F::cast_from(0.21932454224643019154e-1_f64) * t25429 * t23329 * t88035 * t14165 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t25442 * t23728 - F::cast_from(0.36554090374405031923e-2_f64) * t25429 * t25442 * t23594 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t83352 * t7553 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t88050 * t23332;
    t88054
}
