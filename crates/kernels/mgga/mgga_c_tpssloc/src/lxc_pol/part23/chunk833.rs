//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 833/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk833<F: Float>(t14507: F, t3038: F, t14506: F, t3199: F, t3185: F, t1654: F, t2394: F) -> (F, F, F, F) {
    let t14511 = t14507 * t3038;
    let t14608 = t14506 * t3199;
    let t14618 = t14506 * t3185;
    let t14702 = t2394 * t1654;
    (t14511, t14608, t14618, t14702)
}
