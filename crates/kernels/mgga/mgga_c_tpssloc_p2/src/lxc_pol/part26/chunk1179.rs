//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1179/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1179<F: Float>(t3252: F, t7286: F, t7285: F, t3248: F, t24574: F, t7288: F, t225: F, t7306: F, t2154: F, t3599: F, t11606: F, t11925: F, t11928: F, t1238: F, t1252: F, t2155: F, t24630: F, t24634: F, t24639: F, t24646: F, t24758: F, t24868: F, t24871: F, t24873: F, t24877: F, t24880: F, t3593: F, t3631: F, t498: F, t7283: F, t7351: F, t7392: F) -> (F, F, F, F, F, F, F) {
    let t24883 = t7286 * t3252;
    let t24884 = t7285 * t24883;
    let t24887 = t7286 * t3248;
    let t24888 = t7285 * t24887;
    let t24891 = t24574 * t7288;
    let t24893 = t7306 * t225;
    let t24896 = t2154 * t3599;
    let t24897 = t11606 * t24896;
    let t24900 = -F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t24630 - F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t24634 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t24639 - t11928 * t2155 - F::cast_from(2.0_f64) * t3593 * t7392 + F::cast_from(0.54831135561607547884e-2_f64) * t24646 - t11925 * t2155 + t24758 * t498 - t1238 * t24868 - t7351 * t3631 + t24871 * t498 + F::cast_from(2.0_f64) * t24873 * t498 + F::cast_from(2.0_f64) * t1238 * t24877 - F::cast_from(2.0_f64) * t24880 * t1252 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t24884 - F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t24888 - F::cast_from(0.18277045187202515961e-2_f64) * t24891 - F::cast_from(2.0_f64) * t24893 * t1252 - F::cast_from(6.0_f64) * t1238 * t24897;
    (t24883, t24884, t24887, t24888, t24893, t24897, t24900)
}
