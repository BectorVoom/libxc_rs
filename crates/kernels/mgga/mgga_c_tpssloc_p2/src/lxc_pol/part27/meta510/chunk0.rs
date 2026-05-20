//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1914/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1914<F: Float>(t1599: F, t6699: F, t1922: F, t4542: F, t1625: F, t6703: F, t6706: F, t7561: F, t986: F, t23365: F, t7565: F, t23336: F, t7553: F) -> (F, F, F, F, F, F, F) {
    let t25400 = t1599 * t6699;
    let t25403 = t4542 * t1922;
    let t25406 = t6703 * t1625;
    let t25407 = t25406 * t6706;
    let t25410 = t986 * t7561;
    let t25413 = t23365 * t7565;
    let t25416 = t23336 * t7553;
    (t25400, t25403, t25406, t25407, t25410, t25413, t25416)
}
