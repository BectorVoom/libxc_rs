//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 509/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk509<F: Float>(t234: F, t252: F, t776: F, t6637: F, t6552: F, t1905: F, t794: F, t6562: F, t6604: F, t814: F) -> (F, F, F, F, F, F, F, F) {
    let t6638 = t234 * t252;
    let t6639 = t6638 * t776;
    let t6640 = t6637 * t6639;
    let t6641 = t6552 * t6640;
    let t6643 = t794 * t1905;
    let t6644 = t6562 * t6643;
    let t6645 = 0.41123351671205660912e-2 * t6644;
    let t6646 = t6604 * t814;
    (t6638, t6639, t6640, t6641, t6643, t6644, t6645, t6646)
}
