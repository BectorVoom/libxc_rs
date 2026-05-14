//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 992/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk992<F: Float>(t23966: F, t9239: F, t2240: F, t240: F, t33: F, t1860: F, t1864: F, t67: F, t835: F, t80743: F, t81281: F, t81072: F, t81074: F, t80825: F, t80847: F, t80885: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t84219 = t9239 * t23966;
    let t84241 = t2240 * t33 * t240;
    let t84280 = 1232.0 / 81.0 * t1860 * t835 * t67 * t1864;
    let t84400 = 0.3244175520728446583e0 * t80743;
    let t84423 = 0.19739208802178717238e0 * t81281;
    let t84480 = 0.55440370401180965083e0 * t81072;
    let t84481 = 0.3244175520728446583e0 * t81074;
    let t84514 = 0.2034786907144675699e0 * t80825;
    let t84520 = 455.0 / 648.0 * t80847;
    let t84533 = 0.67287926823567318088e-4 * t80885;
    (t84219, t84241, t84280, t84400, t84423, t84480, t84481, t84514, t84520, t84533)
}
