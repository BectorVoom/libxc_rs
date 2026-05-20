//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3093/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3093<F: Float>(t1099: F, t1118: F, t63847: F, t63881: F, t63916: F, t64011: F, t64027: F, t64049: F, t64066: F, t64094: F, t3356: F, t6031: F) -> (F, F) {
    let t64100 = F::new(1.0) * t1099 * (t63847 + t63881 + t63916 + t64011 + t64027 + t64049 + t64066 + t64094) * t1118;
    let t64103 = t6031 * t3356;
    (t64100, t64103)
}
