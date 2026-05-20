//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1238/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1238<F: Float>(t13258: F, t9634: F, t9629: F, t2379: F, t2632: F, t776: F, t9975: F, t6589: F, t67: F, t246: F, t232: F, t9458: F) -> (F, F, F, F, F, F) {
    let t41435 = t13258 * t9634;
    let t41437 = t13258 * t9629;
    let t41448 = t2632 * t2379;
    let t41453 = t9975 * t776;
    let t41466 = t6589 * t67;
    let t41467 = t41466 * t246;
    let t41468 = t232 * t9458;
    (t41435, t41437, t41448, t41453, t41467, t41468)
}
