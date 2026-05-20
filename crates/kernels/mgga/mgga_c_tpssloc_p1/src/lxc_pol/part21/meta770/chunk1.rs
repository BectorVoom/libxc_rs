//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2671/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2671<F: Float>(t12461: F, t6463: F, t54312: F, t54314: F, t5356: F, t54316: F, t1307: F, t16018: F, t193: F, t19631: F, t20081: F, t3698: F, t3701: F, t3719: F, t39320: F, t39324: F, t39327: F, t5126: F, t5127: F, t5160: F, t533: F, t571: F) -> (F, F, F, F) {
    let t56136 = t6463 * t12461;
    let t56140 = F::new(48.0) * t54312;
    let t56141 = F::new(96.0) * t54314;
    let t56142 = t5356 * t5356;
    let t56147 = F::new(64.0) * t54316;
    let t56148 = F::new(12.0) * t1307 * t19631 * t5126 * t571 - F::new(2.0) * t193 * t3701 * t533 * t56142 + F::new(12.0) * t16018 * t5126 * t5127 + F::new(6.0) * t20081 * t3719 * t5126 + F::new(2.0) * t3698 * t5160 * t56136 + t39320 - t39324 + t39327 - t56140 + t56141 - t56147;
    (t56140, t56141, t56147, t56148)
}
