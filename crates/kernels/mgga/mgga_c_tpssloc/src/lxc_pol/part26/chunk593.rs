//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 593/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk593<F: Float>(t1143: F, t1147: F, t1146: F, t445: F, t440: F, t1155: F) -> (F, F, F, F) {
    let t3371 = t1143 * t1147;
    let t3374 = t1146 * t445;
    let t3375 = F::new(1.0) / t3374;
    let t3376 = t440 * t3375;
    let t3377 = t1155 * t1155;
    (t3371, t3375, t3376, t3377)
}
