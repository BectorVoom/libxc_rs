//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1284/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1284<F: Float>(t20396: F, t67: F, t758: F, t1390: F, t20675: F, t20531: F, t588: F, t592: F, t172: F, t763: F, t120: F, t20553: F) -> (F, F, F, F, F, F) {
    let t73967 = t20396 * t67 * t758;
    let t74068 = t20675 * t1390;
    let t74072 = t588 * t20531;
    let t74074 = t592 * t20531;
    let t74077 = t20396 * t172 * t763;
    let t74090 = t120 * t20553;
    (t73967, t74068, t74072, t74074, t74077, t74090)
}
