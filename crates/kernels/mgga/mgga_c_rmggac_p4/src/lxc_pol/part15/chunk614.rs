//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 614/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk614<F: Float>(t2350: F, t352: F, t262: F, t7192: F, t22: F, t511: F, t899: F, t2347: F, t321: F, t333: F, t7198: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8635 = t2350 * t352;
    let t8636 = t262 * t8635;
    let t8637 = t7192 * t8636;
    let t8639 = t511 * t22;
    let t8640 = t899 * t8639;
    let t8641 = t2347 * t321;
    let t8642 = t262 * t8641;
    let t8643 = t8640 * t8642;
    let t8645 = t2347 * t333;
    let t8646 = t262 * t8645;
    let t8647 = t7198 * t8646;
    let t8649 = t2347 * t352;
    (t8635, t8636, t8637, t8639, t8640, t8641, t8642, t8643, t8645, t8646, t8647, t8649)
}
