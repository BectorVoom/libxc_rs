//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 762/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk762<F: Float>(t491: F, t7284: F, t225: F, t497: F, t1090: F, t1186: F, t2123: F, t1235: F, t462: F, t457: F, t461: F, t1240: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7285 = t7284 * t491;
    let t7286 = t225 * t497;
    let t7287 = t7286 * t1090;
    let t7288 = t7285 * t7287;
    let t7291 = t1186 * t2123;
    let t7294 = t1235 * t225;
    let t7295 = t7294 * t497;
    let t7296 = t462 * t7295;
    let t7299 = t457 * t461;
    let t7300 = t7299 * t491;
    let t7301 = t225 * t1240;
    (t7285, t7286, t7287, t7288, t7291, t7295, t7296, t7299, t7300, t7301)
}
