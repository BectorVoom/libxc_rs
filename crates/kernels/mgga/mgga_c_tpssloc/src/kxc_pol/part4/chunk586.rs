//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 586/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk586<F: Float>(t3358: F, t3236: F, t1143: F, t1147: F, t1146: F, t445: F) -> (F, F, F, F) {
    let t3359 = F::new(1.0) / t3358;
    let t3363 = F::cast_from(0.12361111111111111111e-1_f64) * t3236;
    let t3371 = t1143 * t1147;
    let t3374 = t1146 * t445;
    let t3375 = F::new(1.0) / t3374;
    (t3359, t3363, t3371, t3375)
}
