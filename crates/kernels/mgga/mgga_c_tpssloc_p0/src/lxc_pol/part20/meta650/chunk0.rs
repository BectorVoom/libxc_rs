//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2390/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2390<F: Float>(t13634: F, t49039: F, t13615: F, t2798: F, t896: F, t2815: F, t13623: F, t2807: F, t10588: F, t4378: F, t13629: F, t48981: F, t894: F) -> (F, F, F, F, F, F, F) {
    let t49043 = t13634 * t49039;
    let t49049 = t2798 * t13615 * t896;
    let t49052 = t2815 * t13615 * t896;
    let t49054 = t13623 * t2807;
    let t49056 = t4378 * t10588;
    let t49058 = t13629 * t2807;
    let t49060 = t894 * t48981;
    (t49043, t49049, t49052, t49054, t49056, t49058, t49060)
}
