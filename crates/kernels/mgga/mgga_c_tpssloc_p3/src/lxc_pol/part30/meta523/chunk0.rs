//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1862/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1862<F: Float>(t1874: F, t26179: F, t6525: F, t7458: F, t22751: F, t7692: F, t22666: F, t7691: F, t6888: F, t5187: F, t6890: F, t6889: F) -> (F, F, F, F, F, F, F) {
    let t26181 = F::cast_from(2.0_f64) * t26179 * t1874;
    let t26183 = F::cast_from(2.0_f64) * t7458 * t6525;
    let t26184 = t22751 * t7692;
    let t26186 = t22666 * t7691;
    let t26187 = t6888 * t26186;
    let t26189 = t6890 * t5187;
    let t26190 = t6889 * t26189;
    (t26181, t26183, t26184, t26186, t26187, t26189, t26190)
}
