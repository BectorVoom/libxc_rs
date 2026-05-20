//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2651/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2651<F: Float>(t1799: F, t5356: F, t20684: F, t40611: F, t1390: F, t20675: F, t20531: F, t588: F, t592: F, t172: F, t20396: F, t763: F) -> (F, F, F, F, F, F) {
    let t74060 = t1799 * t5356;
    let t74064 = t20684 * t40611;
    let t74068 = t20675 * t1390;
    let t74072 = t588 * t20531;
    let t74073 = F::new(4.0) * t74072;
    let t74074 = t592 * t20531;
    let t74075 = F::new(4.0) * t74074;
    let t74077 = t20396 * t172 * t763;
    (t74060, t74064, t74068, t74073, t74075, t74077)
}
