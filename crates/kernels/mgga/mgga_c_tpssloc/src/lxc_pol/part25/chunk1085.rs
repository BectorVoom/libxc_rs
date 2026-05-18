//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1085/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1085<F: Float>(t22684: F, t6546: F, t22687: F, t131: F, t1365: F, t1878: F, t209: F, t12156: F, t6889: F, t6890: F, t22648: F, t6897: F, t794: F) -> (F, F, F, F, F) {
    let t80727 = t6546 * t22684;
    let t80728 = t80727 * t22687;
    let t80730 = t1365 * t131;
    let t80732 = t1878 * t80730 * t209;
    let t80735 = t80732 * t6889 * t6890 * t12156;
    let t80738 = t6897 * t794 * t22648;
    (t80727, t80728, t80732, t80735, t80738)
}
