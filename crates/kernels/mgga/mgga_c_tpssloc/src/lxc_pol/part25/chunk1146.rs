//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1146/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1146<F: Float>(t59: F, t9971: F, t240: F, t812: F, t9978: F, t6613: F, t9612: F, t831: F, t23040: F, t2617: F, t232: F, t25119: F, t47072: F, t815: F) -> (F, F, F, F) {
    let t81816 = t9971 * t59;
    let t81818 = t812 * t81816 * t240;
    let t81819 = t81818 * t9978;
    let t81821 = t9612 * t6613;
    let t81822 = t81821 * t831;
    let t81824 = t2617 * t23040;
    let t81825 = t81824 * t831;
    let t81829 = t25119 * t815 * t47072 * t232;
    (t81819, t81822, t81825, t81829)
}
