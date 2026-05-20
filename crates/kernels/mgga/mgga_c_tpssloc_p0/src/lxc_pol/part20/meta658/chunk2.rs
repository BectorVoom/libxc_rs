//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2442/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2442<F: Float>(t10402: F, t14618: F, t14608: F, t13969: F, t13981: F, t3130: F, t10422: F, t14129: F, t3070: F, t11002: F, t14508: F, t10895: F, t14511: F) -> (F, F, F, F, F, F) {
    let t49929 = t14618 * t10402;
    let t49934 = t14608 * t10402;
    let t49940 = t3130 * t13969 * t13981;
    let t49945 = t3070 * t10422 * t14129;
    let t49957 = t14508 * t11002;
    let t49959 = t14511 * t10895;
    (t49929, t49934, t49940, t49945, t49957, t49959)
}
