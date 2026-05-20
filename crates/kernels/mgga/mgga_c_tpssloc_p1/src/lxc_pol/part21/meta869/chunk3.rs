//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3185/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3185<F: Float>(t11692: F, t11697: F, t18964: F, t18583: F, t3577: F, t11678: F, t18367: F, t1227: F, t13969: F, t18593: F, t15640: F, t15737: F) -> (F, F, F, F, F) {
    let t66073 = t11692 * t11697 * t18964;
    let t66076 = t3577 * t11697 * t18583;
    let t66079 = t11678 * t11697 * t18367;
    let t66084 = t1227 * t13969 * t18593;
    let t66092 = t15737 * t15640;
    (t66073, t66076, t66079, t66084, t66092)
}
