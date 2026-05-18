//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 704/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk704<F: Float>(t3507: F, t3508: F, t1214: F, t248: F, t1210: F, t3504: F, t3500: F) -> (F, F, F, F) {
    let t3509 = t3507 * t3508;
    let t3511 = t248 * t1214 * t3509;
    let t3514 = t1210 * t3504;
    let t3515 = t3500 * t3514;
    (t3509, t3511, t3514, t3515)
}
