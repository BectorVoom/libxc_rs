//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2287/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2287<F: Float>(t15743: F, t5005: F, t1174: F, t6177: F, t698: F, t11692: F, t11697: F, t18964: F, t18583: F, t3577: F, t11678: F, t18367: F) -> (F, F, F, F, F) {
    let t66054 = t5005 * t15743;
    let t66057 = t1174 * t698 * t6177;
    let t66073 = t11692 * t11697 * t18964;
    let t66076 = t3577 * t11697 * t18583;
    let t66079 = t11678 * t11697 * t18367;
    (t66054, t66057, t66073, t66076, t66079)
}
