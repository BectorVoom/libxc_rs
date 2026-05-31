//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2297/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2297<F: Float>(t2147: F, t7319: F, t11871: F, t15032: F, t24589: F, t24815: F, t24821: F, t24859: F, t27516: F, t27562: F, t3610: F, t7387: F, t8082: F, t85824: F, t85854: F, t86037: F, t86076: F, t86077: F, t94850: F, t94947: F, t94948: F, t94949: F, t94954: F, t94963: F, t94966: F) -> (F, F) {
    let t94976 = t7319 * t2147;
    let t94980 = F::cast_from(0.54831135561607547884e-2_f64) * t85854 - t94947 - F::cast_from(0.54831135561607547884e-2_f64) * t86037 * t94948 * t94949 * t24815 + F::cast_from(0.27415567780803773942e-2_f64) * t86037 * t94954 * t94949 * t24821 + F::cast_from(0.73108180748810063846e-2_f64) * t86076 * t86077 * t94850 + F::cast_from(0.54831135561607547884e-2_f64) * t94963 * t85824 + F::cast_from(0.60923483957341719871e-3_f64) * t94966 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t27516 * t24859 + F::cast_from(2.0_f64) * t15032 * t7387 + F::cast_from(2.0_f64) * t3610 * t8082 * t11871 + F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t94976 * t27562;
    (t94976, t94980)
}
