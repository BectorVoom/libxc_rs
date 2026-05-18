//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1189/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1189<F: Float>(t1799: F, t22633: F, t22635: F, t97608: F, t1985: F, t20661: F, t6889: F, t6906: F, t20416: F, t6888: F, t6890: F, t20465: F, t22833: F) -> (F, F, F, F) {
    let t107031 = t22633 * t22635 * t97608 * t1799;
    let t107044 = t1985 * t6889 * t6906 * t20661;
    let t107056 = t6888 * t6889 * t6890 * t20416;
    let t107063 = t22833 * t20465;
    (t107031, t107044, t107056, t107063)
}
