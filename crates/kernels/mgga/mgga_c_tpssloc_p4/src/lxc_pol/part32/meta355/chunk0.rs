//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1403/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1403<F: Float>(t14506: F, t3032: F, t3129: F, t3038: F, t225: F, t4658: F, t4553: F, t4559: F, t4555: F, t3199: F, t3185: F, t1057: F, t14205: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14507 = t14506 * t3032;
    let t14508 = t14507 * t3129;
    let t14511 = t14507 * t3038;
    let t14529 = t4658 * t225;
    let t14545 = t4553 * t225;
    let t14552 = t4559 * t225;
    let t14555 = t4555 * t225;
    let t14608 = t14506 * t3199;
    let t14618 = t14506 * t3185;
    let t14651 = t14205 * t1057;
    (t14508, t14511, t14529, t14545, t14552, t14555, t14608, t14618, t14651)
}
