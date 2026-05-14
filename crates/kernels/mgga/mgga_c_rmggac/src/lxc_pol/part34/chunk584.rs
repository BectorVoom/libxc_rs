//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 584/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk584<F: Float>(t118: F, t2000: F, t1004: F, t107: F, t490: F, t1326: F, t1330: F, t31: F, t356: F, t640: F, t2164: F, t7556: F, t288: F, t49: F, t108: F, t4179: F) -> (F, F, F, F, F, F, F, F) {
    let t35039 = t2000 * t118;
    let t35154 = t1004 * t107;
    let t35155 = t490 * t35154;
    let t35206 = t1326 * t1330;
    let t35219 = t356 * t31;
    let t35228 = t640 * t35219;
    let t35244 = t2164 * t7556;
    let t35253 = t49 * t288;
    let t35311 = t4179 * t108;
    (t35039, t35154, t35155, t35206, t35228, t35244, t35253, t35311)
}
