//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2099/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2099<F: Float>(t2635: F, t41424: F, t2639: F, t9663: F, t13258: F, t9634: F, t9629: F, t6589: F, t67: F, t246: F, t232: F, t9458: F) -> (F, F, F, F, F, F, F) {
    let t41425 = t41424 * t2635;
    let t41427 = t2639 * t9663;
    let t41435 = t13258 * t9634;
    let t41437 = t13258 * t9629;
    let t41466 = t6589 * t67;
    let t41467 = t41466 * t246;
    let t41468 = t232 * t9458;
    (t41425, t41427, t41435, t41437, t41466, t41467, t41468)
}
