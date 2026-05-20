//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1289/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1289<F: Float>(t17: F, t20396: F, t750: F, t1358: F, t20596: F, t12283: F, t20442: F, t120: F, t20356: F, t20465: F, t16398: F, t20470: F) -> (F, F, F, F, F, F) {
    let t74496 = t17 * t20396 * t750;
    let t74578 = t20596 * t1358;
    let t74584 = t12283 * t20442;
    let t74592 = t120 * t20356;
    let t74597 = t12283 * t20465;
    let t74618 = t16398 * t20470;
    (t74496, t74578, t74584, t74592, t74597, t74618)
}
