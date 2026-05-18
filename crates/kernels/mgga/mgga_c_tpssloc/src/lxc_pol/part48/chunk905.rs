//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 905/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk905<F: Float>(t1894: F, t2553: F, t59: F, t6591: F, t240: F, t241: F, t2627: F, t812: F, t2632: F, t4180: F, t9626: F, t2617: F, t30713: F) -> (F, F, F) {
    let t112788 = t6591 * t1894 * t59 * t2553;
    let t112792 = t812 * t2627 * t240 * t241;
    let t112795 = t112792 * t4180 * t9626 * t2632;
    let t112797 = t2617 * t30713;
    (t112788, t112795, t112797)
}
