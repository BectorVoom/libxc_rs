//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2585/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2585<F: Float>(t3447: F, t44584: F, t4904: F, t44510: F, t14753: F, t15402: F, t14744: F, t1174: F, t135: F, t15359: F, t11589: F, t15293: F) -> (F, F, F, F, F, F) {
    let t51980 = t3447 * t44584 * t4904;
    let t51988 = t3447 * t44510 * t4904;
    let t51991 = t3447 * t15402 * t14753;
    let t51995 = t3447 * t15402 * t14744;
    let t52013 = t1174 * t135 * t15359;
    let t52019 = t3447 * t11589 * t15293;
    (t51980, t51988, t51991, t51995, t52013, t52019)
}
