//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 933/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk933<F: Float>(t11691: F, t11757: F, t11817: F, t11866: F, t493: F, t3493: F, t3612: F, t1245: F, t11812: F, t1243: F, t10471: F, t11715: F, t11712: F, t11720: F, t491: F) -> (F, F, F, F, F, F, F) {
    let t11868 = t11691 + t11757 + t11817 + t11866;
    let t11869 = t493 * t11868;
    let t11871 = t3612 * t3493;
    let t11872 = t1245 * t11871;
    let t11877 = t11812 * t1243;
    let t11880 = t10471 * t11715;
    let t11881 = t11712 * t11880;
    let t11882 = t491 * t11720;
    (t11868, t11869, t11871, t11872, t11877, t11881, t11882)
}
