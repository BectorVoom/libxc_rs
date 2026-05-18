//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 892/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk892<F: Float>(t32159: F, t32184: F, t533: F, t1390: F, t6999: F, t8804: F, t3701: F, t7216: F, t2095: F, t671: F, t8774: F, t2075: F, t7056: F) -> (F, F, F, F, F, F, F) {
    let t32185 = t32159 + t32184;
    let t32186 = t533 * t32185;
    let t32187 = t32186 * t1390;
    let t32189 = t8804 * t6999;
    let t32193 = t3701 * t7216;
    let t32194 = t2095 * t32193;
    let t32197 = t8774 * t671;
    let t32200 = t2075 * t7056;
    (t32185, t32186, t32187, t32189, t32194, t32197, t32200)
}
