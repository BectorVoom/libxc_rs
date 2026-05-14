//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1104/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1104<F: Float>(t12541: F, t12543: F, t1396: F, t1398: F, t1404: F, t16513: F, t16515: F, t16548: F, t1852: F, t1858: F, t20149: F, t20152: F, t20158: F, t20186: F, t5364: F, t5381: F, t580: F, t6471: F, t6483: F) -> (F,) {
    let tv3rho32 = t1396 * t6483 + t1398 * t20186 + t1404 * t6471 + 2.0 * t1852 * t5381 + 2.0 * t1858 * t5364 + t20149 * t580 + t12541 + t12543 + t16513 + t16515 + t16548 + 2.0 * t20152 + t20158;
    (tv3rho32,)
}
