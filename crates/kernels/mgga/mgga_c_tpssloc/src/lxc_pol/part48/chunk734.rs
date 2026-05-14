//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 734/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk734<F: Float>(t24887: F, t7285: F, t24574: F, t7288: F, t225: F, t7306: F, t2154: F, t3599: F, t11606: F, t11925: F, t11928: F, t1238: F, t1252: F, t2155: F, t24630: F, t24634: F, t24639: F, t24646: F, t24758: F, t24868: F, t24871: F, t24873: F, t24877: F, t24880: F, t24884: F, t3593: F, t3631: F, t498: F, t7283: F, t7351: F, t7392: F) -> (F,) {
    let t24888 = t7285 * t24887;
    let t24891 = t24574 * t7288;
    let t24893 = t7306 * t225;
    let t24896 = t2154 * t3599;
    let t24897 = t11606 * t24896;
    let t24900 = -0.16449340668482264365e-1 * t7283 * t24630 - 0.54831135561607547884e-2 * t7283 * t24634 + 0.16449340668482264365e-1 * t7283 * t24639 - t11928 * t2155 - 2.0 * t3593 * t7392 + 0.54831135561607547884e-2 * t24646 - t11925 * t2155 + t24758 * t498 - t1238 * t24868 - t7351 * t3631 + t24871 * t498 + 2.0 * t24873 * t498 + 2.0 * t1238 * t24877 - 2.0 * t24880 * t1252 - 0.27415567780803773942e-2 * t7283 * t24884 - 0.54831135561607547884e-2 * t7283 * t24888 - 0.18277045187202515961e-2 * t24891 - 2.0 * t24893 * t1252 - 6.0 * t1238 * t24897;
    (t24900,)
}
