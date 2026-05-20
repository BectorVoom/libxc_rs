//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 805/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk805<F: Float>(t1388: F, t1845: F, t4072: F, t89: F, t22751: F, t7692: F, t22666: F, t7691: F, t6888: F, t5187: F, t6890: F, t6889: F) -> (F, F, F, F, F, F) {
    let t26163 = t1845 * t1388;
    let t26179 = t89 * t4072;
    let t26184 = t22751 * t7692;
    let t26186 = t22666 * t7691;
    let t26187 = t6888 * t26186;
    let t26189 = t6890 * t5187;
    let t26190 = t6889 * t26189;
    (t26163, t26179, t26184, t26187, t26189, t26190)
}
