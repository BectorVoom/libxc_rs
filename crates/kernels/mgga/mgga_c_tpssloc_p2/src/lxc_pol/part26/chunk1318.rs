//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1318/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1318<F: Float>(t82307: F, t870: F, t1914: F, t40772: F, t10140: F, t25: F, t193: F, t9458: F, t10121: F, t22960: F, t46240: F, t1877: F, t1915: F, t1916: F, t22959: F, t23286: F, t23290: F, t23295: F, t23296: F, t23299: F, t23302: F, t4314: F, t606: F, t6670: F, t6671: F, t81521: F, t81525: F, t81529: F, t81539: F, t81543: F, t81548: F, t9257: F) -> (F, F, F, F) {
    let t82308 = t82307 * t870;
    let t82312 = t1914 * t40772;
    let t82313 = t25 * t10140;
    let t82320 = t193 * t9458;
    let t82323 = t25 * t10121;
    let t82330 = t22960 * t46240;
    let t82333 = F::cast_from(3.0_f64) * t1877 * t23295 * t81521 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t81525 * t6671 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t6670 * t81529 - F::cast_from(3.0_f64) * t1877 * t23290 * t23299 + t1877 * t1915 * t9257 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t1877 * t81539 * t23296 + F::cast_from(9.0_f64) * t4314 * t1915 * t81543 - F::cast_from(9.0_f64) * t22959 * t81548 + t1877 * t82308 * t25 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) * t1877 * t82312 * t82313 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t23286 * t606 + F::cast_from(3.0_f64) * t82320 * t1916 - t1877 * t6670 * t82323 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t23290 * t23302 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t22959 * t82330;
    (t82308, t82312, t82320, t82333)
}
