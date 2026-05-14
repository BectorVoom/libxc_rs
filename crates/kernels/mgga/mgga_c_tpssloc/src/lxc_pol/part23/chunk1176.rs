//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1176/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1176<F: Float>(t12283: F, t20465: F, t16398: F, t20470: F, t118: F, t20416: F, t3739: F, t794: F, t16094: F, t16095: F, t6347: F, t686: F, t213: F, t20582: F, t40021: F, t20356: F, t40412: F) -> (F, F, F, F, F, F, F) {
    let t74597 = t12283 * t20465;
    let t74618 = t16398 * t20470;
    let t74702 = t3739 * t118 * t794 * t20416;
    let t74724 = t16094 * t686 * t16095 * t6347;
    let t74726 = t213 * t20416;
    let t74741 = t40021 * t20582;
    let t74745 = t40412 * t118 * t794 * t20356;
    (t74597, t74618, t74702, t74724, t74726, t74741, t74745)
}
