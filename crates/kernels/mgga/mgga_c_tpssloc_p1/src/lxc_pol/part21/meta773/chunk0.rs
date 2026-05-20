//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2675/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2675<F: Float>(t1388: F, t5187: F, t1307: F, t5356: F, t54392: F, t54395: F, t54398: F, t54400: F, t15904: F, t20077: F, t20085: F, t3734: F, t3918: F, t39463: F, t39468: F, t39472: F, t5126: F, t5161: F) -> (F, F, F, F, F) {
    let t56194 = t5187 * t1388;
    let t56198 = t1307 * t5356;
    let t56202 = F::cast_from(0.70178683471615754484e1_f64) * t54392;
    let t56203 = F::cast_from(0.36622894612013090108e-3_f64) * t54395;
    let t56207 = F::new(2.0) * t54398;
    let t56208 = F::new(80.0) * t54400;
    let t56212 = F::new(12.0) * t15904 * t20085 * t3918 - F::new(6.0) * t20077 * t3734 * t5126 - F::new(12.0) * t3918 * t5161 * t56194 - F::new(12.0) * t3918 * t5161 * t56198 + t39463 - t39468 - t39472 + t56202 - t56203 + t56207 + t56208;
    (t56202, t56203, t56207, t56208, t56212)
}
