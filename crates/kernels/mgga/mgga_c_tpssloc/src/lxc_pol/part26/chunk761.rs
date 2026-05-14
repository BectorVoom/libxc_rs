//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 761/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk761<F: Float>(t2519: F, t751: F, t2393: F, t763: F, t2374: F, t2749: F, t2752: F, t702: F, t9454: F, t2411: F) -> (F, F, F, F, F) {
    let t9462 = t2519 * t751;
    let t9463 = 3.0 * t9462;
    let t9467 = t2393 * t763;
    let t9469 = 0.21687162600603479684e-1 * t2374 * t9467;
    let t9470 = t2749 * t2752;
    let t9474 = t9454 * t702;
    let t9476 = 6.0 * t2411 * t9474;
    (t9463, t9467, t9469, t9470, t9476)
}
