//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1333/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1333<F: Float>(t1873: F, t45637: F, t12734: F, t6534: F, t39235: F, t45602: F, t9348: F, t1268: F, t81455: F, t22479: F, t2314: F, t45814: F) -> (F, F, F, F, F, F, F, F) {
    let t83946 = F::cast_from(6.0_f64) * t45637 * t1873;
    let t83948 = F::cast_from(12.0_f64) * t12734 * t6534;
    let t83952 = F::cast_from(2.0_f64) * t39235 * t1873;
    let t83956 = F::cast_from(6.0_f64) * t45602 * t1873;
    let t83958 = F::cast_from(6.0_f64) * t9348 * t6534;
    let t83960 = F::cast_from(2.0_f64) * t1268 * t81455;
    let t83962 = F::cast_from(6.0_f64) * t2314 * t22479;
    let t83964 = F::cast_from(2.0_f64) * t45814 * t1873;
    (t83946, t83948, t83952, t83956, t83958, t83960, t83962, t83964)
}
