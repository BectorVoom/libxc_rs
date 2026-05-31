//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2277/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2277<F: Float>(t10165: F, t1052: F, t1599: F, t17575: F, t17635: F, t17686: F, t17691: F, t23327: F, t23329: F, t23336: F, t23581: F, t25429: F, t25430: F, t25743: F, t25755: F, t28515: F, t4557: F, t4665: F, t5919: F, t6687: F, t6815: F, t6816: F, t7553: F, t88022: F, t88023: F, t88812: F, t88845: F, t88868: F, t88932: F) -> F {
    let t99390 = F::cast_from(0.36554090374405031923e-2_f64) * t25429 * t23329 * t25430 * t17635 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t23336 * t28515 + t88812 + F::cast_from(0.73108180748810063846e-2_f64) * t25429 * t23329 * t25430 * t17691 + F::cast_from(0.8529287754027840782e-2_f64) * t88022 * t23329 * t88023 * t17686 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t23581 * t28515 + F::cast_from(4.0_f64) * t4557 * t25743 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t1599 * t88932 - F::cast_from(6.0_f64) * t1052 * t10165 * t6815 * t5919 + F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t88868 * t7553 - t17575 * t6816 - t88845 + F::cast_from(4.0_f64) * t25755 * t4665;
    t99390
}
