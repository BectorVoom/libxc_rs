//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 771/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk771<F: Float>(t15391: F, t68541: F, t15067: F, t68490: F, t15376: F, t68524: F, t14117: F, t21708: F, t9137: F, t15336: F, t68528: F, t217: F, t3119: F, t597: F, t7715: F) -> (F, F, F, F, F, F) {
    let t73922 = t68541 * t15391;
    let t73924 = t68490 * t15067;
    let t73926 = t68524 * t15376;
    let t73929 = t21708 * t14117 * t9137;
    let t73931 = t68528 * t15336;
    let t73935 = t217 * t597 * t7715 * t3119;
    (t73922, t73924, t73926, t73929, t73931, t73935)
}
