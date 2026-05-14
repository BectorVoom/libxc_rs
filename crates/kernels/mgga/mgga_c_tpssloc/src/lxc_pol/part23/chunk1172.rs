//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1172/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1172<F: Float>(t1390: F, t20675: F, t20531: F, t588: F, t592: F, t172: F, t20396: F, t763: F, t120: F, t20553: F, t12283: F, t20454: F, t20489: F, t16398: F, t20475: F, t20460: F) -> (F, F, F, F, F, F, F, F, F) {
    let t74068 = t20675 * t1390;
    let t74072 = t588 * t20531;
    let t74074 = t592 * t20531;
    let t74077 = t20396 * t172 * t763;
    let t74090 = t120 * t20553;
    let t74110 = t12283 * t20454;
    let t74120 = t120 * t20489;
    let t74147 = t16398 * t20475;
    let t74189 = t12283 * t20460;
    (t74068, t74072, t74074, t74077, t74090, t74110, t74120, t74147, t74189)
}
