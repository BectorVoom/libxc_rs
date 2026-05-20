//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2060/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2060<F: Float>(t31: F, t717: F, t607: F, t707: F, t9862: F, t2617: F, t9670: F, t9973: F, t236: F, t40931: F, t10021: F, t812: F, t815: F) -> (F, F, F, F, F, F) {
    let t41284 = t31 * t717;
    let t41291 = t707 * t9862 * t607;
    let t41340 = t2617 * t9670;
    let t41344 = t2617 * t9973;
    let t41347 = t40931 * t236;
    let t41362 = t812 * t815 * t10021;
    (t41284, t41291, t41340, t41344, t41347, t41362)
}
