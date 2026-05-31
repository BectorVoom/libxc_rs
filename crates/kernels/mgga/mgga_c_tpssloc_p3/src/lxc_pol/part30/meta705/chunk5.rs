//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2313/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2313<F: Float>(t1022: F, t1058: F, t1060: F, t1615: F, t17686: F, t17691: F, t18138: F, t23346: F, t23613: F, t23633: F, t23635: F, t23670: F, t25429: F, t25510: F, t25721: F, t28593: F, t28618: F, t28622: F, t28637: F, t28652: F, t3186: F, t3966: F, t6800: F, t7619: F, t82625: F, t82799: F, t88022: F, t89071: F, t89176: F) -> F {
    let t100377 = -F::cast_from(0.21932454224643019154e-1_f64) * t25429 * t25510 * t89071 * t17686 + F::cast_from(0.73108180748810063846e-2_f64) * t25429 * t25510 * t25721 * t17691 + F::cast_from(0.8529287754027840782e-2_f64) * t88022 * t25510 * t89176 * t17686 - F::cast_from(0.36554090374405031923e-2_f64) * t25429 * t23613 * t28637 - F::cast_from(0.21932454224643019153e-1_f64) * t23670 * t28622 + F::cast_from(4.0_f64) * t3186 * t7619 * t18138 + F::cast_from(0.54831135561607547884e-2_f64) * t23633 * t82625 * t28652 + F::cast_from(0.54831135561607547884e-2_f64) * t23633 * t23635 * t3966 * t1615 * t6800 + t1058 * t28593 * t1022 * t1060 + t82799 + F::cast_from(0.14621636149762012769e-1_f64) * t23346 * t28618;
    t100377
}
