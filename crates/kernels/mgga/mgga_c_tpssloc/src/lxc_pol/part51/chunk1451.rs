//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1451/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1451<F: Float>(t122119: F, t122137: F, t122155: F, t122174: F, t122196: F, t122223: F, t122240: F, t122255: F, t122285: F, t122299: F, t122319: F, t122349: F, t122375: F, t122396: F, t122547: F, t122576: F, t1390: F, t1983: F, t533: F) -> F {
    let t122583 = t1983 * t533 * (t122119 + t122137 + t122155 + t122174 + t122196 + t122223 + t122240 + t122255 + t122285 + t122299 + t122319 + t122349 + t122375 + t122396 + t122547 + t122576) * t1390;
    t122583
}
