//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 570/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk570<F: Float>(t1176: F, t461: F, t491: F, t225: F, t497: F, t1090: F, t1186: F, t2123: F, t1235: F, t462: F, t457: F, t1240: F, t1251: F, t1190: F, t2144: F, t1193: F, t2127: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7284 = t1176 * t461;
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
    let t7302 = t7301 * t1251;
    let t7303 = t7300 * t7302;
    let t7306 = t1190 * t2144;
    let t7309 = t2127 * t1193 / 288.0;
    (t7284, t7285, t7286, t7287, t7288, t7291, t7295, t7296, t7299, t7300, t7301, t7302, t7303, t7306, t7309)
}
