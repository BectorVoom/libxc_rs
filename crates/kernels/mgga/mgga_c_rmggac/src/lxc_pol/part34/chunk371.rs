//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 371/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk371<F: Float>(t2100: F, t7638: F, t262: F, t7617: F, t2103: F, t2115: F, t2118: F, t344: F, t830: F, t1173: F, t2189: F, t2064: F, t321: F, t201: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7639 = t2100 * t7638;
    let t7640 = 0.18183107769496894486e-1 * t7639;
    let t7645 = t262 * t7617;
    let t7646 = t2103 * t7645;
    let t7647 = 0.24244143692662525982e-1 * t7646;
    let t7651 = t2115 * t7638;
    let t7652 = 0.4838420607177634088e-3 * t7651;
    let t7656 = t2118 * t7645;
    let t7662 = t344 * t830;
    let t7663 = 0.64905642291407286545e-3 * t7662;
    let t7690 = t2189 * t1173;
    let t7707 = t2064 * t321;
    let t7715 = t201 * t1173;
    (t7639, t7640, t7645, t7646, t7647, t7651, t7652, t7656, t7662, t7663, t7690, t7707, t7715)
}
