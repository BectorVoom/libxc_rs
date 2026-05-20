//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1974/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1974<F: Float>(t15292: F, t15330: F, t15386: F, t15423: F, t225: F, t3507: F, t475: F, t6739: F, t1755: F, t11546: F, t14726: F, t15026: F, t3032: F) -> (F, F, F, F, F, F) {
    let t15425 = t15292 + t15330 + t15386 + t15423;
    let t15426 = t15425 * t225;
    let t15429 = t6739 * t3507 * t475;
    let t15430 = t1755 * t15429;
    let t15434 = t11546 * t14726;
    let t15437 = t15026 * t3032;
    (t15425, t15426, t15429, t15430, t15434, t15437)
}
