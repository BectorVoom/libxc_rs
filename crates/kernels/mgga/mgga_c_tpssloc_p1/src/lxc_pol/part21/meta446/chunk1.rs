//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1994/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1994<F: Float>(t12652: F, t4972: F, t4582: F, t11153: F, t3584: F, t14165: F, t1734: F, t3508: F) -> (F, F, F, F, F, F) {
    let t15649 = t4972 * t12652;
    let t15650 = t4582 * t15649;
    let t15654 = t3584 * t11153;
    let t15655 = t15654 * t14165;
    let t15656 = t4582 * t15655;
    let t15659 = t1734 * t3508;
    (t15649, t15650, t15654, t15655, t15656, t15659)
}
