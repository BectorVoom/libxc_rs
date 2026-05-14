//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1012/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1012<F: Float>(t10097: F, t22986: F, t2647: F, t6646: F, t22641: F, t2588: F, t225: F, t814: F, t6648: F, t23021: F, t6547: F, t23155: F, t23168: F, t6552: F, t6637: F, t6638: F, t9516: F) -> (F, F, F, F, F, F) {
    let t81610 = t22986 * t6646 * t10097 * t2647;
    let t81612 = t22641 * t2588;
    let t81613 = t225 * t814;
    let t81615 = t81612 * t81613 * t6648;
    let t81617 = t6547 * t23021;
    let t81623 = t23168 * t23155;
    let t81627 = t6552 * t6637 * t6638 * t9516;
    (t81610, t81613, t81615, t81617, t81623, t81627)
}
