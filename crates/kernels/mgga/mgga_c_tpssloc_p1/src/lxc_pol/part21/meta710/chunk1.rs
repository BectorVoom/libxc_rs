//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2545/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2545<F: Float>(t10422: F, t14040: F, t3070: F, t10516: F, t4640: F, t10403: F, t14121: F, t13748: F, t2960: F, t13965: F, t3114: F, t14202: F, t3117: F) -> (F, F, F, F, F, F) {
    let t49666 = t3070 * t10422 * t14040;
    let t49678 = t4640 * t10516;
    let t49682 = t10403 * t10422 * t14121;
    let t49684 = t2960 * t13748;
    let t49690 = t3114 * t13965;
    let t49692 = t3117 * t14202;
    (t49666, t49678, t49682, t49684, t49690, t49692)
}
