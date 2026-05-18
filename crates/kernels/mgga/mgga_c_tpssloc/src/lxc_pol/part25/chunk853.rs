//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 853/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk853<F: Float>(t1049: F, t3040: F, t3188: F, t10857: F, t381: F, t1060: F, t1022: F, t3166: F, t10947: F, t3185: F, t3199: F, t3196: F, t4684: F) -> (F, F, F, F, F, F, F) {
    let t11023 = t1049 * t3040;
    let t11024 = t11023 * t3188;
    let t11027 = t381 * t10857;
    let t11028 = t11027 * t1060;
    let t11030 = t3166 * t1022;
    let t11031 = t11030 * t1060;
    let t11034 = t10947 * t3185;
    let t11037 = t10947 * t3199;
    let t11040 = t3196 * t4684;
    (t11023, t11024, t11028, t11031, t11034, t11037, t11040)
}
