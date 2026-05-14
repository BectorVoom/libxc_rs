//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 977/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk977<F: Float>(t1539: F, t3132: F, t3071: F, t3041: F, t1616: F, t2776: F, t13969: F, t4584: F, t1041: F, t4589: F, t12652: F, t4583: F, t4582: F, t12648: F, t13559: F, t977: F) -> (F, F, F, F, F, F, F, F) {
    let t14121 = t1539 * t3132;
    let t14122 = t3071 * t14121;
    let t14125 = t1539 * t3041;
    let t14126 = t3071 * t14125;
    let t14129 = t1616 * t2776;
    let t14130 = t3071 * t14129;
    let t14134 = t13969 * t4584;
    let t14136 = t1041 * t14134 / 1728.0;
    let t14137 = t13969 * t4589;
    let t14139 = 5.0 / 10368.0 * t1041 * t14137;
    let t14142 = t4583 * t12652;
    let t14143 = t4582 * t14142;
    let t14146 = t4583 * t12648;
    let t14147 = t4582 * t14146;
    let t14152 = t977 * t13559;
    (t14122, t14126, t14130, t14136, t14139, t14143, t14147, t14152)
}
