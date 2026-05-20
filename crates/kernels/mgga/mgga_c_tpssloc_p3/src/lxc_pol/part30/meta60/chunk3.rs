//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 408/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk408<F: Float>(t1086: F, t1111: F, t1092: F, t1103: F, t1108: F, t1115: F) -> (F, F, F) {
    let t1150 = F::new(0.301925e0) * t1086;
    let t1153 = F::new(0.82785e-1) * t1111;
    let t1155 = F::new(0.258925e1) * t1103 - t1150 + F::new(0.301925e0) * t1092 + F::new(0.16504875e0) * t1108 - t1153 + F::new(0.82785e-1) * t1115;
    (t1150, t1153, t1155)
}
