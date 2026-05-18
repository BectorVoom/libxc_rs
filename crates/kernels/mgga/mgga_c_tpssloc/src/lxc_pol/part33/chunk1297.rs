//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1297/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1297<F: Float>(t28525: F, t344: F, t6740: F, t5904: F, t6764: F, t1933: F, t23479: F, t99665: F, t1015: F, t23472: F, t28586: F, t17615: F, t6717: F) -> (F, F, F, F, F) {
    let t99720 = t6740 * t28525 * t344;
    let t99731 = t5904 * t6764;
    let t99774 = t1933 * t99665 * t23479;
    let t99779 = t23472 * t1015 * t28586;
    let t99785 = t6717 * t17615;
    (t99720, t99731, t99774, t99779, t99785)
}
