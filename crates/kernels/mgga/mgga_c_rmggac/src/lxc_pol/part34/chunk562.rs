//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 562/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk562<F: Float>(t15377: F, t15380: F, t15389: F, t15392: F, t15395: F, t15400: F, t15406: F, t15412: F, t2211: F, t8975: F, t739: F, t8946: F, t884: F, t8041: F, t8936: F, t1356: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15661 = 0.58171619854173713846e-5 * t15377;
    let t15662 = 0.58171619854173713846e-5 * t15380;
    let t15663 = 0.35038612185802734376e-6 * t15389;
    let t15664 = 0.35038612185802734376e-6 * t15392;
    let t15665 = 0.1276937996798935182e-4 * t15395;
    let t15666 = 0.72714524817717142308e-5 * t15400;
    let t15667 = 0.85129199786595678799e-5 * t15406;
    let t15668 = 0.58171619854173713846e-5 * t15412;
    let t15669 = t2211 * t8975;
    let t15670 = t739 * t15669;
    let t15671 = 0.11974241701863808564e0 * t15670;
    let t15672 = t2211 * t8946;
    let t15673 = t884 * t15672;
    let t15674 = 0.11974241701863808564e0 * t15673;
    let t15675 = t8041 * t8936;
    let t15676 = t1356 * t15675;
    (t15661, t15662, t15663, t15664, t15665, t15666, t15667, t15668, t15669, t15671, t15672, t15674, t15675, t15676)
}
