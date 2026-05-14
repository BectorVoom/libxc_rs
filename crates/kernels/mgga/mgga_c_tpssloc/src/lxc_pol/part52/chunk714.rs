//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 714/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk714<F: Float>(t6889: F, t6891: F, t6888: F, t117: F, t534: F, t67: F, t6559: F) -> (F, F, F, F) {
    let t6892 = t6889 * t6891;
    let t6893 = t6888 * t6892;
    let t6896 = t534 * t67 * t117;
    let t6897 = t6559 * t6896;
    (t6892, t6893, t6896, t6897)
}
