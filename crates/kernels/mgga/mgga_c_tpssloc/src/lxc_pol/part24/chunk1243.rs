//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1243/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1243<F: Float>(t12012: F, t6888: F, t6889: F, t6890: F, t22674: F, t22892: F, t22916: F, t22716: F, t6908: F, t22751: F, t22930: F, t22917: F) -> (F, F, F, F, F) {
    let t80656 = t6888 * t6889 * t6890 * t12012;
    let t80659 = t22892 * t22674 * t22916;
    let t80663 = t22716 * t6908;
    let t80665 = t22751 * t22930;
    let t80667 = t22751 * t22917;
    (t80656, t80659, t80663, t80665, t80667)
}
