//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 310/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk310<F: Float>(t1147: F, t440: F, t1086: F, t1111: F, t448: F) -> (F, F, F, F) {
    let t1148 = t440 * t1147;
    let t1150 = F::new(0.301925e0) * t1086;
    let t1153 = F::new(0.82785e-1) * t1111;
    let t1156 = F::new(1.0) / t448;
    (t1148, t1150, t1153, t1156)
}
