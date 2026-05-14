//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 960/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk960<F: Float>(t1020: F, t17611: F, t10372: F, t10377: F, t10381: F, t10385: F, t1046: F, t13750: F, t13758: F, t13767: F, t13946: F, t17593: F, t17596: F, t17599: F, t17602: F, t17607: F, t973: F) -> (F,) {
    let t17612 = t1020 * t17611;
    let t17614 = -t973 * t17593 / 144.0 + t973 * t17596 / 216.0 + t973 * t17599 / 108.0 + 7.0 / 648.0 * t973 * t17602 - t13750 + t10372 / 2592.0 + t10377 + t10381 / 162.0 + t10385 + t17607 * t1046 / 4608.0 + t13758 + t13767 - t13946 + t17612 / 4608.0;
    (t17614,)
}
