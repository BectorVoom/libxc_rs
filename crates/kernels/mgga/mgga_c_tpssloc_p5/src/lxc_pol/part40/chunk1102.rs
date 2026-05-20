//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1102/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1102<F: Float>(t17161: F, t2979: F, t10214: F, t17152: F, t1040: F, t5904: F, t248: F, t3101: F, t5867: F, t1020: F, t10372: F, t10377: F, t10381: F, t10385: F, t1046: F, t13750: F, t13758: F, t13767: F, t13946: F, t17593: F, t17596: F, t973: F) -> F {
    let t17599 = t2979 * t17161;
    let t17602 = t10214 * t17152;
    let t17607 = t5904 * t1040;
    let t17611 = t248 * t3101 * t5867;
    let t17612 = t1020 * t17611;
    let t17614 = -t973 * t17593 / F::new(144.0) + t973 * t17596 / F::new(216.0) + t973 * t17599 / F::new(108.0) + F::new(7.0) / F::new(648.0) * t973 * t17602 - t13750 + t10372 / F::new(2592.0) + t10377 + t10381 / F::new(162.0) + t10385 + t17607 * t1046 / F::new(4608.0) + t13758 + t13767 - t13946 + t17612 / F::new(4608.0);
    t17614
}
