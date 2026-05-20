//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2525/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2525<F: Float>(t4542: F, t698: F, t973: F, t10186: F, t13788: F, t13560: F, t699: F, t2403: F, t4392: F, t13646: F, t1553: F, t9709: F) -> (F, F, F, F, F, F) {
    let t48066 = t973 * t698 * t4542;
    let t48068 = t10186 * t13788;
    let t48087 = t699 * t13560;
    let t48096 = t2403 * t4392;
    let t48098 = t699 * t13646;
    let t48103 = t9709 * t1553;
    (t48066, t48068, t48087, t48096, t48098, t48103)
}
