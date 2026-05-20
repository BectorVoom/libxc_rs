//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2080/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2080<F: Float>(t91310: F, t26245: F, t80791: F, t26271: F, t80836: F, t1361: F, t22690: F, t22792: F, t5187: F, t1307: F, t7708: F, t80840: F, t90787: F) -> (F, F, F, F, F) {
    let t91311 = F::cast_from(0.6728792682356731809e-4_f64) * t91310;
    let t91312 = t80791 * t26245;
    let t91323 = t80836 * t26271;
    let t91327 = t22792 * t22690 * t1361 * t5187;
    let t91328 = F::cast_from(0.40372756094140390854e-3_f64) * t91327;
    let t91344 = t80840 * t90787 * t7708 * t1307;
    (t91311, t91312, t91323, t91328, t91344)
}
