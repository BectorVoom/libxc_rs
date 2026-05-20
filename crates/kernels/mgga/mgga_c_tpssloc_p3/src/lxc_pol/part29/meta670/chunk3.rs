//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2243/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2243<F: Float>(t22788: F, t5310: F, t16150: F, t6952: F, t16155: F, t26271: F, t80836: F, t1361: F, t22690: F, t22792: F, t5187: F, t16148: F, t26288: F) -> (F, F, F, F, F, F) {
    let t91317 = t22788 * t5310;
    let t91319 = t6952 * t16150;
    let t91321 = t6952 * t16155;
    let t91323 = t80836 * t26271;
    let t91327 = t22792 * t22690 * t1361 * t5187;
    let t91328 = F::cast_from(0.40372756094140390854e-3_f64) * t91327;
    let t91330 = t26288 * t1361 * t16148;
    (t91317, t91319, t91321, t91323, t91328, t91330)
}
