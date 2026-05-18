//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 652/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk652<F: Float>(t4789: F, t49: F, t288: F, t325: F, t4616: F, t235: F, t3807: F, t511: F, t2189: F, t7228: F, t3350: F, t201: F, t4443: F) -> (F, F, F, F, F, F, F, F) {
    let t34795 = t4789 * t49;
    let t34796 = t34795 * t288;
    let t34812 = t325 * t4616;
    let t34813 = t235 * t34812;
    let t34828 = t3807 * t511;
    let t34846 = t2189 * t7228;
    let t34847 = t34846 * t3350;
    let t34855 = t201 * t4443;
    (t34795, t34796, t34812, t34813, t34828, t34846, t34847, t34855)
}
