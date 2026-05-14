//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 582/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk582<F: Float>(t132: F, t1327: F, t140: F, t673: F, t465: F, t7472: F, t7344: F, t7552: F, t4789: F, t49: F, t288: F, t325: F, t4616: F, t235: F, t3807: F, t511: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34750 = t132 * t1327;
    let t34759 = t673 * t140;
    let t34760 = t465 * t34759;
    let t34761 = t7472 * t34760;
    let t34786 = t7344 * t7552;
    let t34795 = t4789 * t49;
    let t34796 = t34795 * t288;
    let t34812 = t325 * t4616;
    let t34813 = t235 * t34812;
    let t34828 = t3807 * t511;
    (t34750, t34760, t34761, t34786, t34795, t34796, t34812, t34813, t34828)
}
